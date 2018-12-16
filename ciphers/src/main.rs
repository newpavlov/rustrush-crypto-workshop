use structopt::StructOpt;
use aes::Aes128;
use generic_array::{GenericArray, typenum::U16};
use block_modes::{Ecb, Cbc, BlockMode, BlockModeIv};
use block_modes::block_padding::Pkcs7;
use ctr::Ctr128;
use cfb_mode::Cfb;
use stream_cipher::{NewStreamCipher, SyncStreamCipher};
use hmac::Hmac;
use pmac::Pmac;
use sha2::Sha256;
use png::HasParameters;

use std::path::Path;
use std::io::{self, Read, Write, BufWriter};
use std::fs::File;

type Aes128CbcPkcs7 = Cbc<Aes128, Pkcs7>;
type Key = GenericArray<u8, U16>;
type Iv = GenericArray<u8, U16>;

mod cli;
use self::cli::Cli;

mod etm;

/// Derive 256 bit master key from password using PBKDF2-HMAC-SHA256
fn derive_master_key(password: &str) -> [u8; 32] {
    let mut buf = [0u8; 32];
    // we use a fixed salt to simplify things
    let salt = b"not very random salt";
    pbkdf2::pbkdf2::<Hmac<Sha256>>(password.as_bytes(), salt, 10_000, &mut buf);
    buf
}

/// Derive 128-bit key and IV from the `password` using PBKDF2-HMAC-SHA256 and
/// HKDF.
fn derive_key_iv(password: &str) -> (Key, Iv) {
    // TODO
    (Key::default(), Iv::default())
}

/// Encrypt content of the `src` file using AES128-CBC with PKCS7 padding and
/// store result in the `dst`.
fn cbc_encrypt_file(password: &str, src: &Path, dst: &Path) -> io::Result<()> {
    // TODO
    Ok(())
}

/// Decrypt content of the `src` file using AES128-CBC with PKCS7 padding and
/// store result in the `dst`. Return `io::ErrorKind::InvalidData` if padding
/// is incorrect.
fn cbc_decrypt_file(password: &str, src: &Path, dst: &Path) -> io::Result<()> {
    // TODO
    Ok(())
}

/// Encrypt/decrypt content of the `src` file using AES128-CTR and
/// store result in the `dst`.
fn ctr_encrypt_decrypt_file(password: &str, src: &Path, dst: &Path) -> io::Result<()> {
    // TODO
    Ok(())
}

/// Encrypt image data using AES128-ECB, assume that `image.len() % 16 == 0`
/// and ignore padding bytes.
fn ecb_image_encrypt(image: &[u8]) -> Vec<u8> {
    // TODO
    vec![]
}

/// Encrypt image data using AES128-CBC, assume that `image.len() % 16 == 0`
/// and ignore padding bytes.
fn cbc_image_encrypt(image: &[u8]) -> Vec<u8> {
    // TODO
    vec![]
}

fn save_png(path: &Path, image: &[u8], info: &png::OutputInfo) -> io::Result<()> {
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, info.width, info.height);
    encoder.set(info.color_type).set(info.bit_depth);
     let mut writer = encoder.write_header()?;
    writer.write_image_data(image)?;
    Ok(())
}

fn encrypt_image(path: &Path) -> io::Result<()> {
    let decoder = png::Decoder::new(File::open(path)?);
    let (info, mut reader) = decoder.read_info()?;
    assert_eq!(info.buffer_size() % 16, 0);

    // raw image data
    let mut image = vec![0; info.buffer_size()];
    reader.next_frame(&mut image)?;

    let ecb_image = ecb_image_encrypt(&image);
    let cbc_image = cbc_image_encrypt(&image);

    save_png(&path.with_file_name("ecb.png"), &ecb_image, &info)?;
    save_png(&path.with_file_name("cbc.png"), &cbc_image, &info)?;

    Ok(())
}

fn main() -> io::Result<()> {
    let opt = Cli::from_args();
    match &opt {
        Cli::CbcEncrypt { password, src, dst } =>
            cbc_encrypt_file(password, src, dst),
        Cli::CbcDecrypt { password, src, dst } =>
            cbc_decrypt_file(password, src, dst),
        Cli::EncryptImage { image } => encrypt_image(image),
        Cli::Ctr { password, src, dst } =>
            ctr_encrypt_decrypt_file(password, src, dst),
        Cli::AesCfbHmacEncrypt { password, src, dst } => {
            let master_key = derive_master_key(password);
            etm::etm_encrypt::<Cfb<Aes128>, Hmac<Sha256>>(
                &master_key, src, dst
            )
        },
        Cli::AesCfbHmacDecrypt { password, src, dst } => {
            let master_key = derive_master_key(password);
            etm::etm_decrypt::<Cfb<Aes128>, Hmac<Sha256>>(
                &master_key, src, dst
            )
        },
        Cli::AesCtrPmacEncrypt { password, src, dst } => {
            let master_key = derive_master_key(password);
            etm::etm_encrypt::<Ctr128<Aes128>, Pmac<Aes128>>(
                &master_key, src, dst
            )
        },
        Cli::AesCtrPmacDecrypt { password, src, dst } => {
            let master_key = derive_master_key(password);
            etm::etm_decrypt::<Ctr128<Aes128>, Pmac<Aes128>>(
                &master_key, src, dst
            )
        },
        Cli::AesPmacSivEncrypt { password, src, dst } => {
            let master_key = derive_master_key(password);
            // TODO
            let _ = (src, dst, master_key);
            Ok(())
        },
        Cli::AesPmacSivDecrypt { password, src, dst } => {
            let master_key = derive_master_key(password);
            //TODO
            let _ = (src, dst, master_key);
            Ok(())
        },
    }
}
