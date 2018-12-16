//! #Examples
//! ```sh
//! rustrush-pwhash pbkdf2 my_salt my_password
//! ceQDgNQEkik+bmUXyPZUsuvP0xTJaAcWSJvRR4Ryb2I=
//! ```
//!
//! ```sh
//! rustrush-pwhash argon2 my_salt my_password
//! 0ebFEUWTY+Y+FaTeA/jbIP4Ofc83lnlk76Bol+CkPk8=
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
            run_pbkdf2(password, salt.as_bytes(), *iterations, *length)
        },
        Cli::Argon2 { password, salt } => {
            run_argon2(password, salt)
        },
    };
    println!("{}", base64::encode(&hash));
}
