mod fingerprint;
mod helpers;
mod data;

#[cfg(test)]
mod tests {
    use std::time::Instant;
    #[test]
    fn test_calc_fingerprint_collection_short() {
        let start_time = Instant::now();
        let fingerprint_handler = super::fingerprint::FingerprintHandle::new();
        let fingerprint_collection = fingerprint_handler
            .calc_fingerprint_collection(
                &super::helpers::decode_mp3_from_file(&format!("./assets/sample.mp3")).unwrap(),
            )
            .unwrap();
        for fingerprint in fingerprint_collection.iter() {
            if let Some(fingerprint) = fingerprint {
                if *fingerprint != 0 {
                    let fingerprint_log10 = (*fingerprint as f64).log10();
                    assert_eq!(
                        fingerprint_log10 > 12_f64 && fingerprint_log10 < 13_f64,
                        true
                    );
                }
            }
        }
        println!(
            "Decoding and hashing stream took {} milliseconds",
            start_time.elapsed().as_millis()
        );
        println!(
            "Number of fingerprints in collection is {}",
            &fingerprint_collection.len()
        );
    }
    #[test]
    // #[ignore]
    fn test_calc_fingerprint_collection_long() {
        let start_time = Instant::now();
        let fingerprint_handler = super::fingerprint::FingerprintHandle::new();
        let fingerprint_collection = fingerprint_handler
            .calc_fingerprint_collection(
                &super::helpers::decode_mp3_from_file(&format!(
                    "./assets/red_hot_chili_peppers_dark_necessities.mp3"
                ))
                .unwrap(),
            )
            .unwrap();
        for fingerprint in fingerprint_collection.iter() {
            if let Some(fingerprint) = fingerprint {
                if *fingerprint != 0 {
                    let fingerprint_log10 = (*fingerprint as f64).log10();
                    assert_eq!(
                        fingerprint_log10 > 12_f64 && fingerprint_log10 < 13_f64,
                        true
                    );
                }
            }
        }
        println!(
            "Decoding and hashing stream took {} milliseconds",
            start_time.elapsed().as_millis()
        );
        println!(
            "Number of fingerprints in collection is {}",
            &fingerprint_collection.len()
        );
    }
}
