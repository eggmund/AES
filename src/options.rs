use structopt::StructOpt;
use std::path::PathBuf;
use std::str::FromStr;
use std::ffi::OsStr;
use std::string::ToString;


#[derive(StructOpt)]
#[structopt(name = "aese")]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
/// Encrypt and Decrypt files with 128 bit AES.
pub struct Opt {
    /// Operation to do (encrypt/decrypt).
    #[structopt(parse(from_os_str))]
    pub operation: Operation,

    /// Type to en/decrypt. Use "aese <type> --help" to view help for each data type.
    #[structopt(subcommand)]
    pub data_type: DataType,

    /// Key to use. If key is longer than 16 characters, only the first 16 characters will be used.
    /// If the key is less than 16 characters, it will be padded with 0s.
    #[structopt(name = "key", short = "k")]
    pub key: String,
}

#[derive(StructOpt)]
pub enum Operation {
    #[structopt(name = "encrypt")]
    Encrypt,
    #[structopt(name = "decrypt")]
    Decrypt,
}

impl FromStr for Operation {
    type Err = ParseOperationDidNotMatch;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "encrypt" => Ok(Operation::Encrypt),
            "decrypt" => Ok(Operation::Decrypt),
            _ => Err(ParseOperationDidNotMatch)
        }
    }
}

impl From<&OsStr> for Operation {
    fn from(os_str: &OsStr) -> Operation {
        let as_str = os_str.to_str().unwrap();
        Self::from_str(as_str).unwrap()
    }
}

#[derive(Debug)]
pub struct ParseOperationDidNotMatch;

impl ToString for ParseOperationDidNotMatch {
    fn to_string(&self) -> String {
        String::from("Parsing operation failed, did not match 'encrypt' or 'decrypt'.")
    }
}

#[derive(StructOpt)]
pub enum DataType {
    #[structopt(name = "file")]
    File {
        /// Path to en/decrypt.
        #[structopt(parse(from_os_str))]
        input_path: PathBuf,

        /// Output path.
        #[structopt(short = "o", long)]
        output_path: PathBuf,
    },
    #[structopt(name = "string")]
    String {
        /// String to en/decrypt. Produces a base64 output if encrypting, and decodes base64 to be decrypted when decrypting.
        #[structopt(parse(from_str))]
        input_string: String,
    },
}

// #[derive(StructOpt, Debug)]
// pub struct SharedOptions {
//     /// Key to use. If key is longer than 16 characters, only the first 16 characters will be used.
//     /// If the key is less than 16 characters, it will be padded with 0s.
//     #[structopt(name = "key", short = "k")]
//     pub key: String,
// }

// #[derive(StructOpt, Debug)]
// pub struct SharedFileOptions {
//     /// File to encrypt/decrypt.
//     #[structopt(name = "path", parse(from_os_str))]
//     pub file_path: PathBuf,

//     /// Output path.
//     #[structopt(name = "output path", short = "o")]
//     pub output_path: PathBuf,
// }

// #[derive(StructOpt, Debug)]
// pub struct SharedStringOptions {
//     /// String to encrypt/decrypt.
//     #[structopt(name = "string", parse(from_os_str))]
//     pub string_in: String,
// }

// #[derive(StructOpt, Debug)]
// #[structopt(name = "aese")]
// #[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
// /// Encrypt and Decrypt files with 128 bit AES.
// pub enum Opt {
//     #[structopt(name = "encrypt")]
//     Encrypt {
//         #[structopt(flatten)]
//         shared_opts: SharedOptions,
//         #[structopt(flatten)]
//         path_opts: SharedFileOptions,
//     },
//     #[structopt(name = "decrypt")]
//     Decrypt {
//         #[structopt(flatten)]
//         shared_opts: SharedOptions,
//         #[structopt(flatten)]
//         path_opts: SharedFileOptions,
//     },
//     #[structopt(name = "encryptstring")]
//     EncryptString {
//         #[structopt(flatten)]
//         shared_opts: SharedOptions,
//         #[structopt(flatten)]
//         string_opts: SharedStringOptions,
//     }
// }