//! This app computes hash sum of a given file (or stdin input) using selected
//! cryptographic hashing algorithm.
//!
//! # Examples
//! ```sh
//! $ rustrush-hashes sha256 ../*
//! ../Cargo.lock   b94363eab7f77787553a6e4f0d4d850b3568499bc8b82e96d6862ebb9865e5a1
//! ../Cargo.toml   221ee6c6b68ce462571e50d0f5b86bb203ff3325e9a493004161727388759359
//! ../hashes   Is a directory (os error 21)
//! ../target   Is a directory (os error 21)
//! ```
//!
//! ```
//! $ echo "hello!" | ../target/release/rustrush-hashes sha256
//! -   c8a31cb076b21999bd2cdcfa5f446a7a6644de88037087112fa18bd90cc13984
//! ```
//!
//! ```sh
//! $rustrush-hashes blake2s src/*
//! src/cli.rs  cff1c6a8d3ee184df95a6c67c853d3b722353fc7b8ed17a31899cc2128cf8c93
//! src/main.rs d93959320b3fc1e778fa43bfae0c6eba5c7993ec3e018eb03d34705710fe7292
//! ```
//!
//! ```sh
//! $ rustrush-hashes var_blake2s 5 src/*
//! src/cli.rs  758b18db3a
//! src/main.rs 0877bb5e65
//! ```
//!
//! ```sh
//! $ rustrush-hashes shake256 5 src/*
//! src/cli.rs  2f71d5e195
//! src/main.rs 70db9c2cae
//! ```
use std::path::{PathBuf, Path};
use std::io;

use structopt::StructOpt;
use digest::{Digest, Input, VariableOutput, ExtendableOutput};
use sha2::Sha256;
use sha3::Shake256;
use blake2::{Blake2b, Blake2s, VarBlake2b, VarBlake2s};

mod cli;
use self::cli::Cli;

fn print_hash(path: Option<&Path>, hash: io::Result<Vec<u8>>) {
    match path {
        Some(path) => print!("{}\t", path.display()),
        None => print!("-\t"),
    }
    match hash {
        Ok(hash) => println!("{}", hex::encode(hash)),
        Err(err) => println!("{}", err),
    }
}

/// Process reader content using hash `Digest` impl and return
/// fixed size hash
fn fixed_digest<D, R>(mut src: R) -> io::Result<Vec<u8>>
    where D: Digest, R: io::Read
{
    // TODO
    Ok(vec![])
}

/// Process reader content using hash `Input` impl and return hash with the
/// requested length. Panic if provided length is not supported.
fn var_digest<D, R>(mut src: R, length: usize) -> io::Result<Vec<u8>>
    where D: Input + VariableOutput, R: io::Read
{
    // TODO
    Ok(vec![])
}

/// Process reader content using hash `io::Write` impl and return hash value
/// with the request length
fn xof_digest<D, R>(mut src: R, length: usize) -> io::Result<Vec<u8>>
    where D: Default +  io::Write + ExtendableOutput, R: io::Read
{
    // TODO
    Ok(vec![])
}

fn process_fixed<D: Digest>(paths: &Vec<PathBuf>) {
    if paths.len() == 0 {
        let res = fixed_digest::<D, _>(io::stdin());
        print_hash(None, res);
    } else {
        for path in paths.iter() {
            let res = std::fs::File::open(path).and_then(fixed_digest::<D, _>);
            print_hash(Some(path), res);
        }
    }
}

fn process_var<D>(paths: &Vec<PathBuf>, length: usize)
    where D: Input + VariableOutput
{
    if let Err(err) = D::new(length) {
        eprintln!("{}", err);
        return;
    }
    if paths.len() == 0 {
        let res = var_digest::<D, _>(io::stdin(), length);
        print_hash(None, res);
    } else {
        for path in paths.iter() {
            let res = std::fs::File::open(path)
                .and_then(|f| var_digest::<D, _>(f, length));
            print_hash(Some(path), res);
        }
    }
}

fn process_xof<D>(paths: &Vec<PathBuf>, length: usize)
    where D: Default + io::Write + ExtendableOutput
{
    if paths.len() == 0 {
        let res = xof_digest::<D, _>(io::stdin(), length);
        print_hash(None, res);
    } else {
        for path in paths.iter() {
            let res = std::fs::File::open(path)
                .and_then(|f| xof_digest::<D, _>(f, length));
            print_hash(Some(path), res);
        }
    }
}

fn main() {
    let opt = cli::Cli::from_args();
    match &opt {
        Cli::Sha256 { path } => process_fixed::<Sha256>(path),
        Cli::Blake2b { path } => process_fixed::<Blake2b>(path),
        Cli::Blake2s { path } => process_fixed::<Blake2s>(path),
        Cli::VarBlake2s { path, length } => process_var::<VarBlake2s>(path, *length),
        Cli::VarBlake2b { path, length } => process_var::<VarBlake2b>(path, *length),
        Cli::Shake256 { path, length } => process_xof::<Shake256>(path, *length),
    };
}
