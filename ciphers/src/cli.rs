use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt)]
#[structopt(
    name = "rustrush-ciphers",
    about = "Various cipher examples")]
pub(crate) enum Cli {
    /// Encrypt file using AES128-CBC-PKCS7 (without any authentification!!!)
    #[structopt(name = "cbc_encrypt")]
    CbcEncrypt {
        /// Password
        password: String,
        /// Path to input (ciphertext) file
        src: PathBuf,
        /// Path to output (plaintext) file
        dst: PathBuf,
    },
    /// Decrypt file using AES128-CBC-PKCS7 (without any authentification!!!)
    #[structopt(name = "cbc_decrypt")]
    CbcDecrypt {
        /// Password
        password: String,
        /// Path to input (ciphertext) file
        src: PathBuf,
        /// Path to output (plaintext) file
        dst: PathBuf,
    },
    /// Encrypt PNG image using AES128-CBC and AES128-ECB and save result as
    /// two images. `color_channels*number_of_pixels` for image should be
    /// divisable by 16.
    #[structopt(name = "image")]
    EncryptImage {
        /// Path to a PNG image
        image: PathBuf,
    },
    /// Encrypt/decrypt file using AES128-CTR (without any authentification!!!)
    #[structopt(name = "ctr")]
    Ctr {
        /// Password
        password: String,
        /// Path to input (ciphertext) file
        src: PathBuf,
        /// Path to output (plaintext) file
        dst: PathBuf,
    },
    /// Encrypt file using AES128-CTR-HMAC
    #[structopt(name = "aes128_ctr_hmac_encrypt")]
    AesCtrHmacEncrypt {
        /// Password
        password: String,
        /// Path to input (ciphertext) file
        src: PathBuf,
        /// Path to output (plaintext) file
        dst: PathBuf,
    },
    /// Encrypt file using AES128-CTR-HMAC
    #[structopt(name = "aes128_ctr_hmac_decrypt")]
    AesCtrHmacDecrypt {
        /// Password
        password: String,
        /// Path to input (ciphertext) file
        src: PathBuf,
        /// Path to output (plaintext) file
        dst: PathBuf,
    },
    /// Encrypt file using AES128-CTR-PMAC
    #[structopt(name = "aes128_ctr_pmac_encrypt")]
    AesCtrPmacEncrypt {
        /// Password
        password: String,
        /// Path to input (ciphertext) file
        src: PathBuf,
        /// Path to output (plaintext) file
        dst: PathBuf,
    },
    /// Encrypt file using AES128-CTR-PMAC
    #[structopt(name = "aes128_ctr_pmac_decrypt")]
    AesCtrPmacDecrypt {
        /// Password
        password: String,
        /// Path to input (ciphertext) file
        src: PathBuf,
        /// Path to output (plaintext) file
        dst: PathBuf,
    },
    /// Encrypt file using AES128-CTR-PMAC
    #[structopt(name = "aes128_pmac_siv_encrypt")]
    AesPmacSivEncrypt {
        /// Password
        password: String,
        /// Path to input (ciphertext) file
        src: PathBuf,
        /// Path to output (plaintext) file
        dst: PathBuf,
    },
    /// Encrypt file using AES128-CTR-PMAC
    #[structopt(name = "aes128_pmac_siv_decrypt")]
    AesPmacSivDecrypt {
        /// Password
        password: String,
        /// Path to input (ciphertext) file
        src: PathBuf,
        /// Path to output (plaintext) file
        dst: PathBuf,
    },
}
