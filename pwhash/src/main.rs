//! #Examples
//! ```sh
//! rustrush-pwhash pbkdf2 aGVsbG8gd29ybGQ= my_password
//! xHI7YUlzLnmPUt9entwsYWIGwZvhWZi5D5SMBffckA0=
//! ```
//!
//! ```sh
//! rustrush-pwhash argon2 random_salt my_password
//! xWodifxnj+eIT2iTCll6ZkYHd7ly4JAxZZ5FV3asfHw=1
//! ```
use structopt::StructOpt;
use hmac::Hmac;
use sha2::Sha256;

mod cli;
use self::cli::Cli;

/// Compute password hash using PBKDF2-HMAC-SHA256
fn run_pbkdf2(
    password: &str, salt: &[u8], iterations: usize, length: usize,
) -> Vec<u8> {
    // TODO
    vec![]
}

/// Compute password hash using Argon2i
fn run_argon2(salt: &str, password: &str) -> Vec<u8> {
    // TODO
    vec![]
}

fn main() {
    let opt = cli::Cli::from_args();
    let hash = match &opt {
        Cli::Pbkdf2 { salt, password, iterations, length } => {
            let salt = base64::decode(salt).unwrap();
            run_pbkdf2(password, &salt, *iterations, *length)
        },
        Cli::Argon2 { password, salt } => {
            run_argon2(password, salt)
        },
    };
    println!("{}", base64::encode(&hash));
}
