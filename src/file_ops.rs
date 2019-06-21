use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::path::Path;

use crate::aes::{self, key_expansion};

// Multiple of 16
const BUFFER_SIZE: usize = 16_384;

pub fn encrypt_file(from: &Path, to: &Path, key: &[u8; 16]) -> std::io::Result<()> {
    // Open original file
    let message_file = File::open(from.to_str()
        .unwrap_or_else(move || panic!("File to encrypt could not be opened."))
    )?;

    // Get file size and make new buffered reader.
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

    println!("File size: {}, buffer size: {}", file_size, BUFFER_SIZE);
    while bytes_done < file_size {
        let mut buffer_size = BUFFER_SIZE;
        let mut padding_amount = 0;

        if file_size - bytes_done < BUFFER_SIZE {
            buffer_size = file_size - bytes_done;
            padding_amount = 16 - (buffer_size % 16);
        }

        let mut buffer: Vec<u8> = vec![0u8; buffer_size];

        dbg!(bytes_done, file_size - bytes_done);
        buf_reader.read(buffer.as_mut_slice())?;

        if padding_amount != 0 {
            /* Add padding. Adds padding to the end of buffer so it can be split into 16 length
               chunks, in a way so that padding can be removed again in decryption.
               For example, if 4 more bytes are needed for padding, then four "4"s are added to the end.
               e.g [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 ,12] -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 4, 4, 4, 4]
               Then when decrypting, four "4"s are found in a row in the last block, so are removed.
            */
            for _ in 0..padding_amount {
                buffer.push(padding_amount as u8);
            }
            buffer_size += padding_amount;
        }

        // Encrypt each state in buffer.
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