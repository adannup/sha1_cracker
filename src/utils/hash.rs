use sha1::Digest;

pub fn encode_text(data: &str) -> String {
    hex::encode(sha1::Sha1::digest(data.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;
    const HASH_TO_TEST: &str = "7c6a61c68ef8b9b6b061b28c348bc1ed7921cb53";

    #[test]
    fn hash_match() {
        let wordline = "passw0rd";
        assert_eq!(HASH_TO_TEST, encode_text(wordline));
    }

    #[test]
    fn hash_mismatch() {
        let wordline = "123456";
        assert_ne!(HASH_TO_TEST, encode_text(wordline));
    }
}
