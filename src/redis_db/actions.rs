use redis::{Client, Connection, pipe};
use std::io::{Error as ErrorRet, ErrorKind};
use std::error::Error;
use std::collections::HashMap;
use super::Repository;

pub struct RedisHelper {
    connection: Connection,
}

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
    fn index_many(&mut self, fingerprints: Vec<usize>, song: String) -> Result<(), Box<dyn Error>> {
        let mut pipeline = pipe();
        for fingerprint in fingerprints.iter() {
            pipeline.rpush(*fingerprint, song.clone()).ignore();
        }
        if let Ok(_) = pipeline.query::<usize>(&mut self.connection) {
            return Ok(())
        }
        Err(Box::new(ErrorRet::new(ErrorKind::Other, format!("Index many query cannot be executed"))))
    }

    fn get_many(&mut self, fingerprints: Vec<usize>) -> Result<HashMap<String, usize>, Box<dyn Error>> {
        Err(Box::new(ErrorRet::new(ErrorKind::Other, format!("Get many query cannot be executed"))))
    }
}