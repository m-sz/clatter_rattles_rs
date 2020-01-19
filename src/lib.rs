mod fingerprint;
mod helpers;

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};
    use std::thread::sleep;
    
    #[test]
    fn test_calc_fingerprint_collection_short() {
        let start_time = Instant::now();
        let fingerprint_handler = super::fingerprint::FingerprintHandle::new();
        let fingerprint_collection = fingerprint_handler.calc_fingerprint_collection(
            &super::helpers::decode_mp3(&format!("./assets/sample.mp3")).unwrap()
        ).unwrap();
        for fingerprint in fingerprint_collection.iter() {
            if *fingerprint != 0 {
                assert_eq!(*fingerprint > 10000000001, true);
            }
        }
        println!("Decoding and hashing stream took {} milliseconds", start_time.elapsed().as_millis());
        println!("Number of fingerprints in collection is {}", &fingerprint_collection.len());
    }
    #[test]
    fn test_calc_fingerprint_collection_long() {
        let start_time = Instant::now();
        let fingerprint_handler = super::fingerprint::FingerprintHandle::new();
        let fingerprint_collection = fingerprint_handler.calc_fingerprint_collection(
            &super::helpers::decode_mp3(&format!("./assets/red_hot_chili_peppers_dark_necessities.mp3")).unwrap()
        ).unwrap();
        for fingerprint in fingerprint_collection.iter() {
            if *fingerprint != 0 {
                assert_eq!(*fingerprint > 10000000001, true);
            }
        }
        println!("Decoding and hashing stream took {} milliseconds", start_time.elapsed().as_millis());
        println!("Number of fingerprints in collection is {}", &fingerprint_collection.len());
    }
}
