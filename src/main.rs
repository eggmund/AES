// #![feature(test)]
// extern crate test;

mod aes;
mod file_ops;

use std::path::Path;

fn main() -> std::io::Result<()> {
    // let f = Path::new("./test_files/void.iso");
    // let w = Path::new("./test_files/encrypted/void.enc");
    // let d = Path::new("./test_files/decrypted/void.iso");

    let f = Path::new("./test_files/void.iso");
    let w = Path::new("./test_files/encrypted/void.enc");
    let d = Path::new("./test_files/decrypted/void.iso");

    let key = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

    file_ops::encrypt_file(f, w, &key)?;
    file_ops::decrypt_file(w, d, &key)?;

    Ok(())
}