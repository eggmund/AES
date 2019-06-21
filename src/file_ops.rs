use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::path::Path;

use crate::aes::{self, key_expansion};

// 1 Mebibyte
const BUFFER_SIZE: usize = 1_048_576;

struct BuffersAndSize {
    write_buf: BufWriter<File>,
    read_buf: BufReader<File>,
    file_size: usize,
}

fn prepare_files(read_from: &Path, write_to: &Path) -> std::io::Result<BuffersAndSize> {
    let read_file = File::open(read_from.to_str()
        .unwrap_or_else(move || panic!("File to encrypt could not be opened."))
    )?;
    let size = read_file.metadata().unwrap().len() as usize;

    let write_file = File::create(write_to.to_str()
        .unwrap_or_else(move || panic!("File to encrypt to could not be opened."))
    )?;

    Ok(BuffersAndSize {
        write_buf: BufWriter::with_capacity(BUFFER_SIZE, write_file),
        read_buf: BufReader::with_capacity(BUFFER_SIZE, read_file),
        file_size: size,
    })
}

/*  Padding is added at the end of the file so it can be split into 16 byte chunks.
    For example, if 4 more bytes are needed for padding, then four "4"s are added to the end.
    e.g [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 ,12] -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 4, 4, 4, 4]
    Then when decrypting, four "4"s are found in a row in the last block, so are removed.
*/
pub fn encrypt_file(read_from: &Path, write_to: &Path, key: &[u8; 16]) -> std::io::Result<()> {
    let mut bufs_and_size = prepare_files(read_from, write_to)?;

    // Expand the key
    let expanded_key = key_expansion::expand_key(key);

    // Write encrypted key to start so key can be validated when decrypting.
    let mut key_enc = key.clone();
    aes::encrypt(&mut key_enc, &expanded_key);
    bufs_and_size.write_buf.write(&key_enc)?;

    let mut bytes_done: usize = 0;
    let mut state = [0u8; 16];

    let padding_amount = 16 - (bufs_and_size.file_size % 16);

    while bytes_done < bufs_and_size.file_size {
        //dbg!(bytes_done, file_size - bytes_done);
        bufs_and_size.read_buf.read(&mut state)?;

        if bufs_and_size.file_size - bytes_done <= 16 && padding_amount != 0 {
            for i in 0..padding_amount {
                state[16 - padding_amount + i] = padding_amount as u8;
            }
        }

        aes::encrypt(&mut state, &expanded_key);

        bufs_and_size.write_buf.write(&state)?;
        bytes_done += 16;
    }

    bufs_and_size.write_buf.flush()?;   // Write remaining stuff

    Ok(())
}

pub fn decrypt_file(read_from: &Path, write_to: &Path, key: &[u8; 16]) -> std::io::Result<()> {
    let mut bufs_and_size = prepare_files(read_from, write_to)?;

    // Expand the key
    let expanded_key = key_expansion::expand_key(key);
    let mut state = [0u8; 16];

    // Check key at start of file
    bufs_and_size.read_buf.read(&mut state)?;
    aes::decrypt(&mut state, &expanded_key);

    if state != *key {
        panic!("Invalid key.");
    }

    let mut bytes_done: usize = 16; // Key already checked

    while bytes_done < bufs_and_size.file_size - 16 {
        bufs_and_size.read_buf.read(&mut state)?;
        aes::decrypt(&mut state, &expanded_key);
        //dbg!(state);
        bufs_and_size.write_buf.write(&state)?;
        bytes_done += 16;
    }

    // Last block may have padding
    bufs_and_size.read_buf.read(&mut state)?;
    aes::decrypt(&mut state, &expanded_key);

    // Check for padding by checking for consecutive of the same end value
    let end_val = state[15] as usize;
    let mut end_chunk = state.to_vec();
    if end_val <= 16 {
        let duplicate_area = end_chunk[(16-end_val)..16].to_vec();
        if duplicate_area == vec![end_val as u8; end_val] {
            end_chunk.truncate(16-end_val);
        }
    }

    bufs_and_size.write_buf.write(end_chunk.as_slice())?;
    bufs_and_size.write_buf.flush()?;   // Write remaining stuff

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