use std::error::Error;
use std::fs;

mod utils;
use crate::utils::hash::encode_text;

pub struct Config {
    pub hash: String,
    pub file_path: String,
}

const SHA1_HEX_STRING_LENGTH: usize = 40;

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let hash = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a hash"),
        };

        if hash.len() != SHA1_HEX_STRING_LENGTH {
            return Err("sha1 hash is not valid");
        }

        Ok(Config { hash, file_path })
    }

    pub fn search_hash(self) -> Result<Option<String>, Box<dyn Error>> {
        let wordlist_file = fs::read_to_string(self.file_path)?;

        let result = search(&wordlist_file, self.hash);
        Ok(result.map(|hash| hash.to_owned()))
    }
}

fn search<'a>(wordlist: &'a str, hash: String) -> Option<&'a str> {
    wordlist
        .lines()
        .find(|line| hash.contains(&encode_text(line)))
}
