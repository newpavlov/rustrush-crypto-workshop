//! Encrypt-then-MAC construct.
use generic_array::GenericArray;
use stream_cipher::{StreamCipher, NewStreamCipher};
use crypto_mac::Mac;
use sha2::Sha256;
use generic_array::typenum::Unsigned;

use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

/// Encrypt data using encrypt-then-MAC approach.
///
/// Derives cipher key, cipher IV and MAC key from the `master_key` using HKDF.
pub fn etm_encrypt<C: StreamCipher + NewStreamCipher, M: Mac>(
    master_key: &[u8], src: &Path, dst: &Path,
) -> io::Result<()> {
    // TODO
    Ok(())
}

/// Decrypt data using encrypt-then-MAC approach.
///
/// Derives cipher key, cipher IV and MAC key from the `master_key` using HKDF.
/// Returns `io::ErrorKind::InvalidData` on MAC mismatch.
pub fn etm_decrypt<C: StreamCipher + NewStreamCipher, M: Mac>(
    master_key: &[u8], src: &Path, dst: &Path,
) -> io::Result<()> {
    // TODO
    Ok(())
}
