pub mod actions;
use std::error::Error;
use std::collections::HashMap;

trait Repository {
    /// Index all given fingerprints by adding or updating fingerprint as a key 
    /// and song as a value in to database
    /// 
    /// #Arguments:
    /// * fingerprints - collection of all fingerprints for a given song
    /// * song - author and title of a song in one string
    /// 
    /// # Return - Success if whole query is done successfully or dynamic Error otherwise
    ///  
    fn store(&mut self, fingerprints: &Vec<usize>, song: &String) -> Result<(), Box<dyn Error>>;

    /// Get all fingerprints with corresponding songs list, then calculate hash map collection
    /// by the song title and author as a key and number of fingerprints it occurred as a value
    /// 
    /// # Arguments:
    /// * fingerprints - collection of all fingerprints that We want to match songs against
    /// 
    /// # Returns - Success of Hash map representing match count if query result is success,
    /// or dynamic Error otherwise
    /// 
    fn find_matches(&mut self, fingerprints: &Vec<usize>) -> Result<HashMap<String, usize>, Box<dyn Error>>;
}
