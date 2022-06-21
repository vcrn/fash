use sha1::Digest;

fn encode_hex_str(hash: &[u8]) -> String {
    let mut buf = [0u8; 64];

    match base16ct::upper::encode_str(hash, &mut buf) {
        Ok(hex) => hex.to_string(),
        Err(_) => "Couldn't encode hash to hexadecimal form".to_string(),
    }
}

pub fn compute_hash<T: Digest>(path: &str) -> Result<String, std::io::Error> {
    let bytes = std::fs::read(path)?;
    let mut hasher = T::new();
    hasher.update(bytes);
    let hash = hasher.finalize();

    Ok(encode_hex_str(&hash))
}

#[test]
fn test_compute_hash_sha256() {
    let path = "tests/test_file_to_hash.txt";
    let hash_expected = "A8A2870D9E3FD571C85E54360492636E409C1A20B15631EDBEBEA5DD5AACC859";
    let hex_hash = compute_hash::<::sha2::Sha256>(path).unwrap();
    assert_eq!(hex_hash, hash_expected);
}

#[test]
fn test_compute_hash_sha1() {
    let path = "tests/test_file_to_hash.txt";
    let hash_expected = "442258E76BFDF5FDAA4C93C44916FC532B7DE9D5";
    let hex_hash = compute_hash::<sha1::Sha1>(path).unwrap();
    assert_eq!(hex_hash, hash_expected);
}

#[test]
fn test_compute_hash_md5() {
    let path = "tests/test_file_to_hash.txt";
    let hash_expected = "3799F5CDAD95FCBCAE0A7D05A5347DFD";
    let hex_hash = compute_hash::<md5::Md5>(path).unwrap();
    assert_eq!(hex_hash, hash_expected);
}
