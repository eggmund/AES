#![feature(test)]

extern crate test;

mod aes;
mod file_ops;

use std::path::Path;

fn main() -> std::io::Result<()> {
    file_ops::encrypt_file(
        Path::new("./test_files/city.jpg"),
        Path::new("./test_files/encrypted/city.enc"),
        &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
    )?;

    Ok(())
}