use sha1::Digest;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, error::Error, process};

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
        process::exit(1);
    }

    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        eprintln!("sha1 hash is not valid");
        process::exit(1);
    }

    let wordlist_file = File::open(&args[1]).expect("Failed to open the wordlist file");
    let reader = BufReader::new(wordlist_file);

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line.trim().to_string(),
            Err(_) => continue,
        };

        if hash_to_crack == &hex::encode(sha1::Sha1::digest(line.as_bytes())) {
            println!("Password found: {}", &line);
            return Ok(());
        }
        // println!("Trying password: {}", line);
    }

    println!("password not found in wordlist :(");

    Ok(())
}
