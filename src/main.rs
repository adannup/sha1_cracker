use std::{env, process};

use sha1_cracker::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        eprintln!("Usage: \n sha1_cracker: <wordlist.txt> <sha1_hash>");
        process::exit(1);
    });

    match config.search_hash() {
        Ok(Some(hash)) => println!("Hash found in wordlist {hash}"),
        Ok(None) => println!("No hash found"),
        Err(e) => {
            eprintln!("Application error: {e}");
            std::process::exit(1);
        }
    }
}
