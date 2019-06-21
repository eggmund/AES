use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::path::Path;

use crate::aes::{self, key_expansion};

// Multiple of 16
const BUFFER_SIZE: usize = 16_384;

pub fn encrypt_file(from: &Path, to: &Path, key: &[u8; 16]) -> std::io::Result<()> {
    let message_file = File::open(from.to_str()
        .unwrap_or_else(move || panic!("File to encrypt could not be opened."))
    )?;

    let file_size = message_file.metadata().unwrap().len() as usize;
    let mut buf_reader = BufReader::new(message_file);

    let cipher_file = File::create(to.to_str()
        .unwrap_or_else(move || panic!("File to encrypt to could not be opened."))
    )?;

    let mut buf_writer = BufWriter::new(cipher_file);

    // Expand the key
    let expanded_key = key_expansion::expand_key(key);

    // Write encrypted key to start so key can be validated when decrypting.
    let mut key_enc = key.clone();
    aes::encrypt(&mut key_enc, &expanded_key);
    buf_writer.write(&key_enc)?;

    let mut bytes_done: usize = 0;

    //let mut state = [0u8; 16];

    println!("File size: {}, buffer size: {}", file_size, BUFFER_SIZE);
    while bytes_done < file_size {
        let mut buffer_size = BUFFER_SIZE;
        let mut padding_amount = 0;

        if file_size - bytes_done < BUFFER_SIZE {
            buffer_size = file_size - bytes_done;
            padding_amount = 16 - (buffer_size % 16);
        }

        let mut buffer: Vec<u8> = vec![0u8; buffer_size];

        println!("DONE: {}\tfile_size: {}\tdiff: {}", bytes_done, file_size, file_size - bytes_done);
        buf_reader.read(buffer.as_mut_slice())?;

        if padding_amount != 0 {
            // Add padding
            for _ in 0..padding_amount {
                buffer.push(padding_amount as u8);
            }
            buffer_size += padding_amount;
        }

        for i in (0..buffer_size).step_by(16) {
            aes::encrypt(&mut buffer[i..i+16], &expanded_key);
        }

        buf_writer.write(&buffer)?;
        bytes_done += buffer_size;
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use test::Bencher;
    use std::path::Path;

    #[bench]
    fn encrypt_file(b: &mut Bencher) {
        b.iter(|| super::encrypt_file(
            Path::new("./test_files/julia.png"),
            Path::new("./test_files/encrypted/julia.enc"),
            &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
        ).unwrap());
    }

    #[bench]
    fn encrypt_large_file(b: &mut Bencher) {
        b.iter(|| super::encrypt_file(
            Path::new("./test_files/loco_roco.iso"),
            Path::new("./test_files/encrypted/loco.enc"),
            &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
        ).unwrap());    }
}