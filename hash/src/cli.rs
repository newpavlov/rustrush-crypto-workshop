use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt)]
#[structopt(
    name = "rustrush-hash",
    about = "Computes hash sum using selected algorithm")]
pub(crate) enum Cli {
    #[structopt(name = "sha256")]
    Sha256 {
        #[structopt(parse(from_os_str))]
        path: Vec<PathBuf>,
    },
    #[structopt(name = "blake2b")]
    Blake2b {
        #[structopt(parse(from_os_str))]
        path: Vec<PathBuf>,
    },
    #[structopt(name = "blake2s")]
    Blake2s {
        #[structopt(parse(from_os_str))]
        path: Vec<PathBuf>,
    },
    #[structopt(name = "var_blake2b")]
    VarBlake2b {
        /// target hash length in bytes
        #[structopt(short = "l", long = "length")]
        length: usize,
        #[structopt(parse(from_os_str))]
        path: Vec<PathBuf>,
    },
    #[structopt(name = "var_blake2s")]
    VarBlake2s {
        /// target hash length in bytes
        #[structopt(short = "l", long = "length")]
        length: usize,
        #[structopt(parse(from_os_str))]
        path: Vec<PathBuf>,
    },
    #[structopt(name = "shake256")]
    Shake256 {
        /// target hash length in bytes
        #[structopt(short = "l", long = "length")]
        length: usize,
        #[structopt(parse(from_os_str))]
        path: Vec<PathBuf>,
    },
}
