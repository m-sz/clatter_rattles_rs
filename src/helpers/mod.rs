use minimp3::{Decoder, Frame};
use std::error::Error;
use std::fs::File;

/// Mp3 decoding function.
///
/// Decoding is done using `minimp3.`
/// Samples are read frame by frame and pushed to the vector.
/// Conversion to mono is done by simply taking the mean of left and right channels.
/// 
/// # Arguments:
/// * filename - path to the mp3 file we want to decode
/// 
/// # Returns - success of decoded frames, dynamic error otherwise
/// 
pub async fn decode_mp3(filename: &str) -> Result<Vec<f32>, Box<dyn Error>> {
    let mut decoder = Decoder::new(File::open(filename)?);
    let mut frames = Vec::new();

    loop {
        match decoder.next_frame() {
            Ok(Frame { data, channels, .. }) => {
                if channels < 1 {
                    return Err(Box::from("Invalid number of channels"));
                }

                for samples in data.chunks_exact(channels) {
                    frames.push(f32::from(
                        samples.iter().fold(0, |sum, x| sum + x / channels as i16),
                    ));
                }
            }
            Err(minimp3::Error::Eof) => break,
            Err(e) => return Err(Box::from(e)),
        }
    }

    Ok(frames)
}