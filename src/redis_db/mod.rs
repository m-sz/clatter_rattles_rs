pub mod actions;
use std::error::Error;
use std::collections::HashMap;

trait Repository {
    /// Index all given fingerprints by adding or updating fingerprint as a key 
    /// and song as a value in to database
    /// 
    /// #Arguments:
    /// * fingerprints - collection of all fingerprints for a given song
    /// * song - song author and title in one string
    /// 
    /// # Return - Success if whole query is done successful or dynamic Error otherwise
    ///  
    fn index_many(&mut self, fingerprints: Vec<usize>, song: String) -> Result<(), Box<dyn Error>>;

    /// Get all fingerprints keys We are asking for with values from the database
    /// and collect the result in HashMap in a way were hash map key is song name and title
    /// and value is a number of all hash keys that holds the song as a value
    /// 
    /// # Arguments:
    /// * fingerprints - collection of all fingerprints that We want to get songs for
    /// 
    /// # Returns - Hash map of song title and author value of counted occurrence of the song along all fingerprints
    /// 
    fn get_many(&mut self, fingerprints: Vec<usize>) -> Result<HashMap<String, usize>, Box<dyn Error>>;
}
