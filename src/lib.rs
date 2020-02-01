mod data;
mod fingerprint;
mod helpers;

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
            "\nDecoding and hashing stream took {} milliseconds\n",
            start_time.elapsed().as_millis()
        );
        println!(
            "\nNumber of fingerprints in collection is {}\n",
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
            "\nDecoding and hashing stream took {} milliseconds\n",
            start_time.elapsed().as_millis()
        );
        println!(
            "\nNumber of fingerprints in collection is {}\n",
            &fingerprint_collection.len()
        );
    }
    use super::data::stream_actions::ArcStreamListener;
    use futures_await_test::async_test;
    use std::thread;
    use std::thread::sleep;
    use std::time::Duration;
    use tokio::runtime::Runtime;

    #[async_test]
    // #[ignore]
    async fn test_stream_listener_mp3_fingerprints() {
        let mut listener = ArcStreamListener::new(
            format!("https://str2b.openstream.co/604?aw_0_1st.collectionid=3162&stationId=3162&publisherId=628&listenerid=1580311050432_0.47836979431904714&awparams=companionAds%3Atrue&aw_0_1st.version=1.1.4%3Ahtml5")
        ).unwrap();
        let receiver = listener.get_listener();
        let reader = thread::spawn(move || {
            let fingerprint_handler = super::fingerprint::FingerprintHandle::new();
            let mut collected_num = 50;
            let mut collected = Vec::new();
            loop {
                let start_time = Instant::now();
                let mut decoded = receiver.recv().unwrap();
                if collected_num < 5 {
                    collected_num += 1;
                    collected.append(&mut decoded);
                } else {
                    collected.append(&mut decoded);
                    let fingerprint_collection = fingerprint_handler
                        .calc_fingerprint_collection(&collected)
                        .unwrap();
                    for fingerprint in fingerprint_collection.iter() {
                        if let Some(fingerprint) = fingerprint {
                            if *fingerprint != 0 {
                                let fingerprint_log10 = (*fingerprint as f64).log10();
                                println!("\nFingerprint for stream chunk: {:?}", &fingerprint);
                                assert_eq!(
                                    fingerprint_log10 > 12_f64 && fingerprint_log10 < 13_f64,
                                    true
                                );
                            }
                        }
                    }
                    collected.clear();
                    println!(
                        "\nDownloading, decoding, pipping and hashing stream mp3 chunk from internet radio took {} milliseconds\n",
                        start_time.elapsed().as_millis()
                    );
                }
            }
        });
        if let Ok(a) = Runtime::new().unwrap().block_on(listener.run_mp3()) {
            sleep(Duration::from_secs(8));
            listener.deactivate();
            a.join().unwrap();
            reader.join().unwrap();
        };
    }
}
