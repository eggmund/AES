// #![feature(test)]
// extern crate test;

mod aes;
mod file_ops;
mod options;

use structopt::StructOpt;

use crate::options::Opt;

fn main() -> std::io::Result<()> {
    let opts = Opt::from_args();

    let key = parse_key_string(opts.key);

    match opts.data_type {
        options::DataType::File {
            input_path,
            output_path,
        } => {
            match opts.operation {
                options::Operation::Encrypt => {
                    file_ops::encrypt_file(&input_path, &output_path, &key)
                },
                options::Operation::Decrypt => {
                    file_ops::decrypt_file(&input_path, &output_path, &key)
                }
            }
        },
        options::DataType::String {
            input_string,
        } => {
            match opts.operation {
                options::Operation::Encrypt => {
                    Ok(())
                },
                options::Operation::Decrypt => {
                    Ok(())
                }
            }
        }
    }
    // match opts {
    //     Opt::Encrypt {
    //         shared_opts,
    //     } => {
    //         let key = parse_key_string(shared_opts.key);
    //         //file_ops::encrypt_file(&shared_opts.file_path, &shared_opts.output_path, &key)
    //         file_ops::encrypt_file(&shared_opts.file_path, &shared_opts.output_path, &key)
    //     },
    //     Opt::Decrypt {
    //         shared_opts,
    //     } => {
    //         let key = parse_key_string(shared_opts.key);
    //         file_ops::decrypt_file(&shared_opts.file_path, &shared_opts.output_path, &key)
    //     },
    // }
}

#[inline]
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
