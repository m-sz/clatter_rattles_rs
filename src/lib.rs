mod data;
mod fingerprint;
mod helpers;

use dotenv;
#[macro_use]
extern crate dotenv_codegen;

mod test {
    use super::{data, data::*, fingerprint, helpers};
    use std::time::Instant;
    use std::fs;
    use rayon::prelude::*;
    use std::sync::{Arc, Mutex};
    // use std::fs::DirEntry;

    #[test]
    fn test_matching_algorithm() -> Result<(), std::io::Error> {
         // NOTE: files names in samples folder should be the same as their counterparts in full_songs folder
         // Please populate folders with your own tracks and samples to test matching algorithm
        let entries = fs::read_dir("assets/most_likely_test/full_songs/")?;
        dotenv::dotenv().ok();
        if dotenv!("REDIS_HAS_SONGS") == "false" {
            println!("Calculating fingerprints and populating database...");
            let start = Instant::now();
            let mut counter = 0;
            let number_of_fingerprints = Arc::new(Mutex::new(0)); 
            let mut file_names = Vec::new(); 
            entries.into_iter().for_each(|entry| {
                if let Ok(entry) = entry {
                    counter += 1;
                    let file_name = entry.file_name().into_string().unwrap().clone();
                    file_names.push(file_name);
                }
            });
            file_names.par_iter().for_each(|file_name| {
                let mut db_handler = data::redis_actions::RedisHelper::new(&"redis://127.0.0.1/").unwrap();
                let fingerprinter = fingerprint::FingerprintHandle::new();
                let decoded = helpers::decode_mp3_from_file(&format!("assets/most_likely_test/full_songs/{}", &file_name)).unwrap();
                let fingerprints = fingerprinter.calc_fingerprint_collection(&decoded).unwrap();
                *number_of_fingerprints.lock().unwrap() += fingerprints.len();
                db_handler.store(&fingerprints.clone(), file_name).unwrap();
            });
            let stop = start.elapsed().as_millis();
            println!("Songs has different time span");
            println!("Total time to calculate and populate db with {} songs took: {} ms", counter, stop);
            println!("Average time to calculate and add to db 1 song {} ms", (stop as f32 / counter as f32 ) as u128);
            println!("Average time to calculate and ad to db 1000 fingerprints took: {} ms", (stop as f32 / (*number_of_fingerprints.lock().unwrap() as f32 / 1000_f32)) as u128);
        }

        // test pick most likely
        println!("Calculating samples fingerprints");
        let entries = fs::read_dir("assets/most_likely_test/samples/")?;
        let start = Instant::now();
        let mut counter = 0;
        let number_of_fingerprints = Arc::new(Mutex::new(0)); 
        let mut file_names = Vec::new(); 
        entries.into_iter().for_each(|entry| {
            if let Ok(entry) = entry {
                counter += 1;
                let file_name = entry.file_name().into_string().unwrap().clone();
                file_names.push(file_name);
            }
        });
        file_names.par_iter().for_each(|file_name| {
            let mut db_handler = data::redis_actions::RedisHelper::new(&"redis://127.0.0.1/").unwrap();
            let fingerprinter = fingerprint::FingerprintHandle::new();
            let decoded = helpers::decode_mp3_from_file(&format!("assets/most_likely_test/samples/{}", &file_name)).unwrap();
            let fingerprints = fingerprinter.calc_fingerprint_collection(&decoded).unwrap();
            *number_of_fingerprints.lock().unwrap() += fingerprints.len();
            let findings = db_handler.find_matches(&fingerprints).unwrap();
            let matched_song = helpers::pick_most_likely(&findings);
            println!("Sample name: {}", &file_name);
            println!("Matched song name {} Match accuracy {} %\n", &matched_song.0, ((matched_song.1 as f32 / fingerprints.len() as f32) * 100_f32) as u16);
        });
        let stop = start.elapsed().as_millis();
        println!("Total time to calculate and match {} samples took: {} ms", counter, stop);
        println!("Average time to calculate fingerprint for 1 sample and match it against fingerprints in db took {} ms", (stop as f32 / counter as f32 ) as u128);
        // assert_eq!(1, 1);
        Ok(())
    }
}
