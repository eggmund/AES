use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::path::Path;

use crate::aes::{self, key_expansion};

// 1 Mebibyte
const BUFFER_SIZE: usize = 1_048_576;

fn prepare_files() {

}

pub fn encrypt_file(from: &Path, to: &Path, key: &[u8; 16]) -> std::io::Result<()> {
    // Open original file
    let message_file = File::open(from.to_str()
        .unwrap_or_else(move || panic!("File to encrypt could not be opened."))
    )?;

    // Get file size and make new buffered reader.
    let file_size = message_file.metadata().unwrap().len() as usize;
    let mut buf_reader = BufReader::with_capacity(BUFFER_SIZE, message_file);

    let cipher_file = File::create(to.to_str()
        .unwrap_or_else(move || panic!("File to encrypt to could not be opened."))
    )?;

    let mut buf_writer = BufWriter::with_capacity(BUFFER_SIZE, cipher_file);

    // Expand the key
    let expanded_key = key_expansion::expand_key(key);

    // Write encrypted key to start so key can be validated when decrypting.
    let mut key_enc = key.clone();
    aes::encrypt(&mut key_enc, &expanded_key);
    buf_writer.write(&key_enc)?;

    let mut bytes_done: usize = 0;
    let mut state = [0u8; 16];

    let padding_amount = 16 - (file_size % 16);

    while bytes_done < file_size {
        //dbg!(bytes_done, file_size - bytes_done);
        buf_reader.read(&mut state)?;

        if file_size - bytes_done <= 16 && padding_amount != 0 {
            /* Add padding. Adds padding to the end of buffer so it can be split into 16 length
               chunks, in a way so that padding can be removed again in decryption.
               For example, if 4 more bytes are needed for padding, then four "4"s are added to the end.
               e.g [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 ,12] -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 4, 4, 4, 4]
               Then when decrypting, four "4"s are found in a row in the last block, so are removed.
            */
            for i in 0..padding_amount {
                state[16 - padding_amount + i] = padding_amount as u8;
            }
        }

        aes::encrypt(&mut state, &expanded_key);

        buf_writer.write(&state)?;
        bytes_done += 16;
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
            Path::new("./test_files/city.jpg"),
            Path::new("./test_files/encrypted/city.enc"),
            &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
        ).unwrap());    
    }
}