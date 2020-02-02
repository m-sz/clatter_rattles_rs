use minimp3::{Decoder, Frame};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

/// Pics the song or acoustic sample that has highest value of matching hashes
///
/// # Arguments:
/// * findings - collection of all songs with matching hash count as a value
///
/// # Returns tuple of best matching song, the one with the highest value
///
#[allow(dead_code)]
pub fn pick_most_likely(findings: &HashMap<String, usize>) -> (String, usize) {
    // TODO: consider returning Option or Result
    let mut best_fit: (String, usize) = (format!("Not found matching song"), 0);
    for (song, val) in findings.iter() {
        if *val > best_fit.1 {
            best_fit.0 = song.clone();
            best_fit.1 = *val;
        }
    }
    best_fit
}

/// Mp3 decoding file function.
///
/// Decoding is done using `minimp3.`
/// Samples are read frame by frame and pushed to the vector.
/// Conversion to mono is done by simply taking the mean of left and right channels.
///
/// # Arguments:
/// * filename - path to the mp3 file we want to decode
///
/// # Returns success of decoded frames, dynamic error otherwise
///
#[allow(dead_code)]
pub fn decode_mp3_from_file(filename: &str) -> Result<Vec<f32>, Box<dyn Error>> {
    let mut decoder = Decoder::new(File::open(filename)?);
    decode_frames(&mut decoder)
}

/// Mp3 decoding stream chunk function.
///
/// Decoding is done using `minimp3.`
/// Samples are read frame by frame and pushed to the vector.
/// Conversion to mono is done by simply taking the mean of left and right channels.
///
/// # Arguments:
/// * chunk - readable chunk of data encoded in mp3 format
///
/// # Returns success of decoded frames, dynamic error otherwise
///
#[allow(dead_code)]
pub fn decode_mp3_from_chunk<R: Read>(chunk: R) -> Result<Vec<f32>, Box<dyn Error>> {
    let mut decoder = Decoder::new(chunk);
    decode_frames(&mut decoder)
}

#[allow(dead_code)]
fn decode_frames<R: Read>(decoder: &mut Decoder<R>) -> Result<Vec<f32>, Box<dyn Error>> {
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

#[cfg(test)]
mod test {
    #[test]
    fn test_decode_mp3_from_file() {
        // This test verifies if used library for decoding mp3 is working fine
        // and nothing substantial has been changed in external lib.
        // Please check always against the same file, otherwise it will not pass.
        let filename = format!("./assets/sample.mp3");
        let decoded_stream = super::decode_mp3_from_file(&filename);
        if let Ok(stream) = decoded_stream {
            println!("\nDecoded bytes: {:?} \n", &stream.len());
            assert_eq!(stream.len(), 619776);
        } else {
            assert_eq!(1, 2);
        }
    }
}
