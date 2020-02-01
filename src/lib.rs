mod data;
mod fingerprint;
mod helpers;

use std::collections::HashMap;

#[allow(dead_code)]
fn pick_most_likely(findings: &HashMap<String, usize>) -> (String, usize) {
    let mut best_fit: (String, usize) = (format!("Not found matching song"), 0);
    for (song, val) in findings.iter() {
        if *val > best_fit.1 {
            best_fit.0 = song.clone();
            best_fit.1 = *val;
        }
    }
    best_fit
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use super::data::redis_actions::RedisHelper;
    use super::data::Repository;
    use super::fingerprint::FingerprintHandle;
    use super::helpers::decode_mp3_from_file;
    use super::pick_most_likely;
    use super::data::stream_actions::ArcStreamListener;
    use futures_await_test::async_test;
    use std::thread;
    use std::thread::sleep;
    use std::time::Duration;
    use tokio::runtime::Runtime;

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
            if *fingerprint != 0 {
                let fingerprint_log10 = (*fingerprint as f64).log10();
                assert_eq!(
                    fingerprint_log10 > 12_f64 && fingerprint_log10 < 13_f64,
                    true
                );
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
            if *fingerprint != 0 {
                let fingerprint_log10 = (*fingerprint as f64).log10();
                assert_eq!(
                    fingerprint_log10 > 12_f64 && fingerprint_log10 < 13_f64,
                    true
                );
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
    fn test_matching_algorithm() {
        let fingerprint_handle = FingerprintHandle::new();
        let mut redis = RedisHelper::new(&"redis://127.0.0.1/").unwrap();
        let path = "./assets/";
        let sample = "sample.mp3"; 
        let files = [
            "Red Hot Chili Peppers - By The Way.mp3",
            "Red Hot Chili Peppers - Californication.mp3",
            "Red Hot Chili Peppers - Can't Stop.mp3",
            "Red Hot Chili Peppers - Give It Away.mp3",
            "Red Hot Chili Peppers - Otherside.mp3",
            "Red Hot Chili Peppers - Snow.mp3",
            "Red Hot Chili Peppers - Wet Sand.mp3",
            "red_hot_chili_peppers_dark_necessities.mp3",
        ];
        for file in files.iter() {
            let path = format!("{}{}", &path, &file);
            let decoded = decode_mp3_from_file(&path).unwrap();
            let fingerprints = fingerprint_handle
                .calc_fingerprint_collection(&decoded)
                .unwrap();
            redis.store(&fingerprints, &format!("{}", file)).unwrap();
        }
        let path = format!("{}{}", &path, &sample);
        let decoded = decode_mp3_from_file(&path).unwrap();
        let fingerprints = fingerprint_handle
            .calc_fingerprint_collection(&decoded)
            .unwrap();
        let findings = redis.find_matches(&fingerprints).unwrap();
        let best_fit = pick_most_likely(&findings);
        println!("{:?}", &best_fit);
    }

    #[async_test]
    // #[ignore]
    async fn test_stream_listener_mp3_fingerprints() {
        let mut listener = ArcStreamListener::new(
            format!("https://str2b.openstream.co/604?aw_0_1st.collectionid=3162&stationId=3162&publisherId=628&listenerid=1580311050432_0.47836979431904714&awparams=companionAds%3Atrue&aw_0_1st.version=1.1.4%3Ahtml5")
        ).unwrap();
        let receiver = listener.get_listener();
        let reader = thread::spawn(move || {
            let fingerprint_handler = super::fingerprint::FingerprintHandle::new();
            let mut collected_num = 0;
            let mut collected = Vec::new();
            loop {
                let start_time = Instant::now();
                let mut decoded = receiver.recv().unwrap();
                if collected_num % 5 == 0 {
                    collected.append(&mut decoded);
                } else {
                    collected.append(&mut decoded);
                    let fingerprint_collection = fingerprint_handler
                        .calc_fingerprint_collection(&collected)
                        .unwrap();
                    for fingerprint in fingerprint_collection.iter() {
                        if *fingerprint != 0 {
                            let fingerprint_log10 = (*fingerprint as f64).log10();
                            println!("\nFingerprint for stream: {:?}", &fingerprint);
                            assert_eq!(
                                fingerprint_log10 > 12_f64 && fingerprint_log10 < 13_f64,
                                true
                            );
                        }
                    }
                    collected.clear();
                    println!(
                        "\nDownloading, decoding, pipping and hashing stream mp3 chunk from internet radio took {} milliseconds\n",
                        start_time.elapsed().as_millis()
                    );
                }
                collected_num += 1;
            }
        });
        if let Ok(a) = Runtime::new().unwrap().block_on(listener.run_mp3()) {
            sleep(Duration::from_secs(10));
            listener.deactivate();
            a.join().unwrap();
            reader.join().unwrap();
        };
    }
}
