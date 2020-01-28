use super::Repository;
use redis::{transaction, Client, Commands, Connection};
use std::collections::{HashMap, HashSet};
use std::error::Error;

pub struct RedisHelper {
    connection: Connection,
}

#[allow(dead_code)]
impl RedisHelper {
    /// Connects to the redis database and set a connection helper
    ///
    /// #Arguments:
    /// * addr - address to the instance of redis database f.e: "redis://127.0.0.1/"
    ///
    /// # Returns - success of Self if connection is established, dynamic Error otherwise
    ///
    pub fn new(addr: &str) -> Result<Self, Box<dyn Error>> {
        let client = Client::open(addr)?;
        let connection = client.get_connection()?;
        Ok(Self { connection })
    }
}

impl Repository for RedisHelper {
    fn store(&mut self, fingerprints: &Vec<usize>, song: &String) -> Result<(), Box<dyn Error>> {
        transaction(&mut self.connection, fingerprints, |con, pipe| {
            for fingerprint in fingerprints.iter() {
                pipe.sadd(*fingerprint, song.clone()).ignore();
            }
            pipe.query(con)
        })?;
        Ok(())
    }

    fn find_matches(
        &mut self,
        fingerprints: &Vec<usize>,
    ) -> Result<HashMap<String, usize>, Box<dyn Error>> {
        let mut matches: HashMap<String, usize> = HashMap::new();
        for fingerprint in fingerprints.iter() {
            let songs: HashSet<String> = self.connection.smembers(*fingerprint)?;
            for song in songs.iter() {
                match matches.get_mut(song) {
                    Some(count) => {
                        *count += 1;
                    }
                    None => {
                        matches.insert(song.clone(), 1);
                    }
                };
            }
        }
        Ok(matches)
    }
}

#[cfg(test)]
mod test {
    use super::{RedisHelper, Repository};
    use std::time::Instant;
    #[test]
    fn test_repository() {
        let fake_fingerprints_1: Vec<usize> = vec![
            1234567890, 1987654321, 1290347856, 1111111111, 2222222222, 3333333333, 4444444444,
            5555555555, 6666666666, 7777777777,
        ];
        let fake_fingerprints_2: Vec<usize> = vec![
            1231231234, 9999999999, 8888888888, 1111111111, 2222222222, 3333333333, 4444444444,
            5555555555, 6666666666, 7777777777,
        ];
        let song_1 = format!("Shrek and Donkey - Pinocchio is laying again");
        let song_2 = format!("Alice in Wonderland - Poker face");
        let mut db_handler = RedisHelper::new(&"redis://127.0.0.1/").unwrap();
        if let Ok(_) = db_handler.store(&fake_fingerprints_1, &song_1) {
            assert_eq!(1, 1);
        } else {
            assert_eq!(1, 2);
        }
        if let Ok(_) = db_handler.store(&fake_fingerprints_2, &song_2) {
            assert_eq!(1, 1);
        } else {
            assert_eq!(1, 2);
        }
        if let Ok(matches) = db_handler.find_matches(&fake_fingerprints_2) {
            match matches.get(&song_1) {
                Some(m) => assert_eq!(*m, 7_usize),
                None => assert_eq!(1, 2),
            };
            match matches.get(&song_2) {
                Some(m) => assert_eq!(*m, 10_usize),
                None => assert_eq!(1, 2),
            };
        } else {
            assert_eq!(1, 2);
        }
    }
    #[test]
    fn test_benchmark_repository() {
        let mut fake_fingerprints_1: Vec<usize> = Vec::new();
        let mut fake_fingerprints_2: Vec<usize> = Vec::new();
        let fingerprint_1_max: usize = 20000;
        let fingerprint_2_max: usize = 30000;
        for fingerprint in 0..fingerprint_1_max {
            fake_fingerprints_1.push(fingerprint);
        }
        for fingerprint in 0..fingerprint_2_max {
            fake_fingerprints_2.push(fingerprint);
        }
        let song_1 = format!("Shrek and Donkey - Pinocchio is laying again");
        let song_2 = format!("Alice in Wonderland - Poker face");
        let mut db_handler = RedisHelper::new(&"redis://127.0.0.1/").unwrap();
        let start_time = Instant::now();
        if let Ok(_) = db_handler.store(&fake_fingerprints_1, &song_1) {
            assert_eq!(1, 1);
        } else {
            assert_eq!(1, 2);
        }
        if let Ok(_) = db_handler.store(&fake_fingerprints_2, &song_2) {
            assert_eq!(1, 1);
        } else {
            assert_eq!(1, 2);
        }
        println!(
            "Adding to database of 2 songs of total {} fingerprints took {} milliseconds",
            fingerprint_2_max,
            start_time.elapsed().as_millis()
        );
        let half_time = Instant::now();
        if let Ok(matches) = db_handler.find_matches(&fake_fingerprints_2) {
            match matches.get(&song_1) {
                Some(m) => assert_eq!(*m, fingerprint_1_max),
                None => assert_eq!(1, 2),
            };
            match matches.get(&song_2) {
                Some(m) => assert_eq!(*m, fingerprint_2_max),
                None => assert_eq!(1, 2),
            };
        } else {
            assert_eq!(1, 2);
        }
        println!(
            "Matching of total {} fingerprints took {} milliseconds",
            fingerprint_1_max + fingerprint_2_max,
            half_time.elapsed().as_millis()
        );
    }
}
