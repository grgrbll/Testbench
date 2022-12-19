
use hex_literal::hex;
use sha2::{Sha256, Sha512, Digest};

#[test]
fn sha() {
    let mut hasher = Sha256::new();

    // write input message
    hasher.update(b"hello world");

    // read hash digest and consume hasher
    let result = hasher.finalize();

    assert_eq!(result[..], hex!("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9")[..]);
}

// todo: aes, rsa