//! This app computes hash sum of a given file (or stdin input) using selected
//! cryptographic hashing algorithm.
//!
//! # Examples
//! ```sh
//! $ ../target/release/rustrush-hash sha256 sha256 ../artwork/*
//! ../artwork/rust-logo-128x128-blk.png    e144e4943ffbeabb97facc3e505dafba2199e27ae1b9bbd57ec65ea3f271cca7
//! ../artwork/rust-logo-128x128-blk-v2.png a3b727c6ff3ff9014b01a9ac0e721b027344a080ca7dc1107533b4c63a369af1
//! ../artwork/rust-logo-128x128.png    be51c2df3133a6e154a0d290adc50d0b6d31f872d80afe8262ac7d280f76363d
//! ../artwork/rust-logo-16x16-blk.png  09e2fee65c3fd07deae4c4253bea484bbe449de7313e32e152d382b31220dee6
//! ../artwork/rust-logo-16x16.png  d811884b5b308621d49441ff57f8d05b43c3c672b296fe898ee30bdc8026f587
//! ../artwork/rust-logo-256x256-blk.png    73eb37d7c8e591ddf6962a10555b58c6926022392269e43ee5d5715397055601
//! ../artwork/rust-logo-256x256.png    3590e5ac1e6ed21915045f7fb6670a4d47558da014c80bd69cb2c9bb548ad10c
//! ../artwork/rust-logo-32x32-blk.png  0f47094c39fd6e4d0505709213c510437967d82afbaa30e78e3ab6586192210a
//! ../artwork/rust-logo-32x32.png  267c0c1bf905fd46fac97f8d8f590c006696af7d093757699230a35822995675
//! ../artwork/rust-logo-512x512-blk.png    18968384b4ab73ea582ae44c81bc63351ad48bfd0ab56a156760c48204473496
//! ../artwork/rust-logo-512x512.png    38c08733444d8673b66e1e9e67420b462dd0e5567bea18d84da7b11d1c8cf118
//! ../artwork/rust-logo-64x64-blk.png  e761ad5a4ff91d9d1dcdf60d5c9cfe175421b83dd1faad93558870e99e6577bf
//! ../artwork/rust-logo-64x64.png  5a7a86bd5a0ec0e65f865dd5a9b2461b21106f5161f57f044c6f348f5a2513b2
//! ../artwork/rust-logo-blk.svg    0ce17af81557923b7097065a579a8319c0719d6545c665f0add9ac0124354265
//! ```
//!
//! ```
//! $ echo "hello!" | ../target/release/rustrush-hash sha256
//! -   c8a31cb076b21999bd2cdcfa5f446a7a6644de88037087112fa18bd90cc13984
//! ```
//!
//! ```sh
//! $ ../target/release/rustrush-hash blake2s ../artwork/rust-logo-128x128-blk.png
//! ../artwork/rust-logo-128x128-blk.png    2d811c5233189b936efbf12ac75dcc124c09ca9b75cd3ca2d711af59e8f293ba
//! ```
//!
//! ```sh
//! $ rustrush-hash var_blake2s --length 5 ../artwork/rust-logo-128x128-blk.png
//! ../artwork/rust-logo-128x128-blk.png    9bfec12802
//! ```
//!
//! ```sh
//! $ rustrush-hash shake256 5 src/*
//! ../artwork/rust-logo-128x128-blk.png    570563a662
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
