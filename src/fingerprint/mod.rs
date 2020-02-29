use rayon::prelude::*;
use rustfft::algorithm::Radix4;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FFT;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::usize;

const FFT_WINDOW_SIZE: usize = 1024; // chunk window size to process by fast forward fourier function
const FREQ_BINS: &[usize] = &[32, 40, 60, 80, 100, 120, 180, 320]; // Each value in array is a top range frequency to calculate local maximum magnitude for
const FUZZ_FACTOR: usize = 8; // higher the value of this factor, lower the fingerprint entropy, and less bias the algorithm become to the sound noises

/// Helper struct for calculating acoustic fingerprint
///
#[allow(dead_code)]
pub struct FingerprintHandle {
    /// FFT algorithm
    fft: Radix4<f32>,
}

#[allow(dead_code)]
impl FingerprintHandle {
    pub fn new() -> FingerprintHandle {
        FingerprintHandle {
            fft: Radix4::new(FFT_WINDOW_SIZE, false),
        }
    }

    /// Calculate fingerprint for decoded stream
    ///
    /// This method uses fast forward fourier computation
    /// to process decoded stream input in to
    /// stream of complex number output,
    /// then calculates fingerprint
    ///
    /// # Arguments:
    /// * decoded_stream - acoustic stream that is decoded to stream of floats
    ///
    /// # Returns success of fingerprint collection, dynamic error otherwise
    ///
    pub fn calc_fingerprint_collection(
        &self,
        decoded_stream: &[f32],
    ) -> Result<Vec<usize>, Box<dyn Error>> {
        let fingerprints_arr: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(Vec::new()));
        decoded_stream
            .par_chunks(FFT_WINDOW_SIZE) // multi threaded iteration over chunks, where chunk of size FFT_WINDOW_SIZE
            .for_each(|chunk| {
                if chunk.len() == FFT_WINDOW_SIZE {
                    let mut input: Vec<Complex<f32>> = chunk.iter().map(Complex::from).collect();
                    let mut output: Vec<Complex<f32>> = vec![Complex::zero(); FFT_WINDOW_SIZE];
                    self.fft.process(&mut input, &mut output);
                    fingerprints_arr
                        .lock()
                        .unwrap()
                        .push(calculate_fingerprint(&output));
                }
            });
        let arr = fingerprints_arr.lock().unwrap().clone();
        Ok(arr)
    }
}

/// Find points with max magnitude in each of the bins
/// 
fn calculate_fingerprint(arr: &[Complex<f32>]) -> usize {
    let mut high_scores: Vec<f32> = vec![0.0; FREQ_BINS.len()];
    let mut record_points: Vec<usize> = vec![0; FREQ_BINS.len()];

    for bin in FREQ_BINS[0]..=FREQ_BINS[FREQ_BINS.len() - 1] {
        let magnitude = arr[bin].re.hypot(arr[bin].im);

        let mut bin_idx = 0;
        while FREQ_BINS[bin_idx] < bin {
            bin_idx += 1;
        }

        if magnitude > high_scores[bin_idx] {
            high_scores[bin_idx] = magnitude;
            record_points[bin_idx] = bin;
        }
    }
    encode(&record_points)
}

/// Encoding function with reverse order
///
fn encode(arr: &[usize]) -> usize {
    (arr[7] as f32 / FUZZ_FACTOR as f32) as usize * usize::pow(10, 16)
    + (arr[6] as f32 / FUZZ_FACTOR as f32) as usize * usize::pow(10, 14)
    + (arr[5] as f32 / FUZZ_FACTOR as f32) as usize * usize::pow(10, 12)
    + (arr[4] as f32 / FUZZ_FACTOR as f32) as usize * usize::pow(10, 10)
    + (arr[3] as f32 / FUZZ_FACTOR as f32) as usize * usize::pow(10, 8)
    + (arr[2] as f32 / FUZZ_FACTOR as f32) as usize * usize::pow(10, 6)
    + (arr[1] as f32 / FUZZ_FACTOR as f32) as usize * usize::pow(10, 2)
    + (arr[0] as f32 / FUZZ_FACTOR as f32) as usize
}
