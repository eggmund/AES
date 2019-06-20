use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::path::Path;

use crate::aes::{self, key_expansion};

const BUFFER_SIZE: u64 = 2048;

pub fn encrypt_file(from: &Path, to: &Path, key: &[u8; 16]) -> std::io::Result<()> {
    let message_file = File::open(from.to_str()
        .unwrap_or_else(move || panic!("File to encrypt could not be opened."))
    )?;

    let file_size = message_file.metadata().unwrap().len();
    println!("Got here");
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

    let mut bytes_done: u64 = 0;
    let mut buffer: [u8; BUFFER_SIZE as usize] = [0u8; BUFFER_SIZE as usize];

    //let mut state = [0u8; 16];

    while bytes_done < file_size - BUFFER_SIZE {
        buf_reader.read(&mut buffer)?;

        for i in (0..BUFFER_SIZE as usize).step_by(16) {
            aes::encrypt(&mut buffer[i..i+16], &expanded_key);
        }

        buf_writer.write(&buffer)?;
        bytes_done += BUFFER_SIZE;
    }

    let padding_amount = (file_size - bytes_done) as u8;
    buf_reader.read(&mut buffer)?;

    for i in 0..padding_amount {
        buffer[(BUFFER_SIZE - padding_amount as u64 + i as u64) as usize] = padding_amount;
    }
    buf_writer.write(&buffer);

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
}