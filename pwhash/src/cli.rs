use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "rustrush-pwhash",
    about = "Password hashing")]
pub(crate) enum Cli {
    #[structopt(name = "pbkdf2")]
    Pbkdf2 {
        /// Number of iterations to perform (complexity)
        #[structopt(short = "i", long = "iterations", default_value = "10000")]
        iterations: usize,
        /// Output hash length in bytes
        #[structopt(short = "l", long = "length", default_value = "32")]
        length: usize,
        salt: String,
        password: String,
    },
    #[structopt(name = "argon2")]
    Argon2 {
        salt: String,
        password: String,
    },
}
