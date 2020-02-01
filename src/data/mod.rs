use super::helpers::decode_mp3_from_chunk;
pub mod redis_actions;
pub mod stream_actions;
use std::collections::HashMap;
use std::error::Error;

trait Repository {
    /// Index all given fingerprints by adding or updating fingerprint as a key
    /// and song as a value in to database
    ///
    /// #Arguments:
    /// * fingerprints - collection of all fingerprints for a given song
    /// * song - author and title of a song in one string
    ///
    /// # Return success if whole query is done successfully or dynamic Error otherwise
    ///  
    fn store(&mut self, fingerprints: &Vec<usize>, song: &String) -> Result<(), Box<dyn Error>>;

    /// Get all fingerprints with corresponding songs list, then calculate hash map collection
    /// by the song title and author as a key and number of fingerprints it occurred as a value
    ///
    /// # Arguments:
    /// * fingerprints - collection of all fingerprints that We want to match songs against
    ///
    /// # Returns success of Hash map representing match count if query result is success,
    /// or dynamic Error otherwise
    ///
    fn find_matches(
        &mut self,
        fingerprints: &Vec<usize>,
    ) -> Result<HashMap<String, usize>, Box<dyn Error>>;
}

trait PlaylistHelper {
    /// Finds uri located inside playlist
    ///
    /// # Return success of uri string if present or Error otherwise
    ///
    fn find_uri(&self) -> Result<String, Box<dyn Error>>;
}
