#![feature(test)]

extern crate test;

mod aes;
mod file_ops;

use std::path::Path;

fn main() -> std::io::Result<()> {
    let f = Path::new("./test_files/city.jpg");
    let w = Path::new("./test_files/encrypted/city.enc");
    let d = Path::new("./test_files/decrypted/city.jpg");

    file_ops::encrypt_file(f, w, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16])?;
    file_ops::decrypt_file(w, d, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16])?;

    Ok(())
}