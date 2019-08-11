use structopt::StructOpt;

use std::path::PathBuf;

#[derive(StructOpt, Debug)]
pub struct SharedOptions {
    /// File to encrypt/decrypt.
    #[structopt(name = "path", parse(from_os_str))]
    pub file_path: PathBuf,

    /// Output path.
    #[structopt(name = "output path", short = "o")]
    pub output_path: PathBuf,

    /// Key to use. If key is longer than 16 characters, only the first 16 characters will be used.
    /// If the key is less than 16 characters, it will be padded with 0s.
    #[structopt(name = "key", short = "k")]
    pub key: String,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "aese")]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
/// Encrypt and Decrypt files with 128 bit AES.
pub enum Opt {
    #[structopt(name = "encrypt")]
    Encrypt {
        #[structopt(flatten)]
        shared_opts: SharedOptions,
    },
    #[structopt(name = "decrypt")]
    Decrypt {
        #[structopt(flatten)]
        shared_opts: SharedOptions,
    },
}