// #![feature(test)]
// extern crate test;

mod aes;
mod file_ops;
mod options;

use structopt::StructOpt;

use std::path::Path;

use crate::options::Opt;

fn main() -> std::io::Result<()> {
    let opts = Opt::from_args();

    match opts {
        Opt::Encrypt {
            shared_opts,
        } => {
            let key = parse_key_string(shared_opts.key);
            file_ops::encrypt_file(&shared_opts.file_path, &shared_opts.output_path, &key)
        },
        Opt::Decrypt {
            shared_opts,
        } => {
            let key = parse_key_string(shared_opts.key);
            file_ops::decrypt_file(&shared_opts.file_path, &shared_opts.output_path, &key)
        },
    }
}

fn parse_key_string(inp: String) -> [u8; 16] {
    let mut inp = inp.into_bytes();
    inp.resize(16, 0u8);

    [
        inp[ 0], inp[ 1], inp[ 2], inp[ 3],
        inp[ 4], inp[ 5], inp[ 6], inp[ 7],
        inp[ 8], inp[ 9], inp[10], inp[11],
        inp[12], inp[13], inp[14], inp[15]
    ]
}