#[cfg(test)]
mod tests {
    const PROGRAM_NAME: &str = "program_name";
    const VALID_FILE_PATH: &str = "wordlist.txt";
    const VALID_HASH: &str = "7c6a61c68ef8b9b6b061b28c348bc1ed7921cb53";
    const INVALID_HASH: &str = "invalid_hash";

    #[test]
    fn build_valid_arguments() {
        let args = vec![
            PROGRAM_NAME.to_string(),
            VALID_FILE_PATH.to_string(),
            VALID_HASH.to_string(),
        ]
        .into_iter();
        let config = sha1_cracker::Config::build(args).unwrap();
        assert_eq!(config.file_path, VALID_FILE_PATH);
        assert_eq!(config.hash, VALID_HASH);
    }

    #[test]
    fn build_missing_file_path() {
        let args = vec![PROGRAM_NAME.to_string()].into_iter();
        let result = sha1_cracker::Config::build(args);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Didn't get a file path"));
    }

    #[test]
    fn build_missing_hash() {
        let args = vec![PROGRAM_NAME.to_string(), VALID_FILE_PATH.to_string()].into_iter();
        let result = sha1_cracker::Config::build(args);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Didn't get a hash"));
    }

    #[test]
    fn build_invalid_hash() {
        let args = vec![
            PROGRAM_NAME.to_string(),
            VALID_FILE_PATH.to_string(),
            INVALID_HASH.to_string(),
        ]
        .into_iter();
        let result = sha1_cracker::Config::build(args);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("sha1 hash is not valid"));
    }

    #[test]
    fn search_hash_successful() {
        let args = vec![
            PROGRAM_NAME.to_string(),
            VALID_FILE_PATH.to_string(),
            VALID_HASH.to_string(),
        ]
        .into_iter();
        let config = sha1_cracker::Config::build(args).unwrap();
        let result = config.search_hash();
        assert!(result.is_ok());

        let hash_result = result.unwrap();
        assert!(hash_result.is_some());
        assert_eq!(hash_result.unwrap(), "passw0rd");
    }

    #[test]
    fn search_hash_no_successful() {
        let args = vec![
            PROGRAM_NAME.to_string(),
            VALID_FILE_PATH.to_string(),
            "7c6a61c68ef8b9b6b061b28c348bc1ed7921cb50".to_string(),
        ]
        .into_iter();
        let config = sha1_cracker::Config::build(args).unwrap();
        let result = config.search_hash();
        assert!(result.is_ok());

        let hash_result = result.unwrap();
        assert!(hash_result.is_none());
    }
}
