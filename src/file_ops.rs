use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter};
use std::path::Path;

use crate::aes::{self, key_expansion};

// 1 Mebibyte
const BUFFER_SIZE: usize = 1_048_576;

/*
pub mod multithreaded {
    use std::thread;
    use std::sync::{mpsc, Arc};
    use std::fs::File;
    use std::io::{self, prelude::*, BufReader, BufWriter};
    use std::path::Path;

    use crate::aes::{self, key_expansion};
    use super::prepare_files;

    struct ThreadPool {
        workers: Vec<thread::JoinHandle<()>>,
        job_sender: mpsc::Sender<Job>,
        result_receiver: mpsc::Receiver<Job>,
    }

    impl ThreadPool {
        fn new(threads: usize) -> ThreadPool {
            let (job_sender, job_receiver) = mpsc::channel();
            let (result_sender, result_receiver) = mpsc::channel();

            let result_sender = Arc::new(result_sender);
            let job_receiver = Arc::new(job_receiver);

            let mut th_pool = ThreadPool {
                workers: Vec::with_capacity(threads),
                job_sender,
                result_receiver,
            }

            for _ in 0..threads {
                workers.push(thread::spawn(move || {
                    loop {
                        
                    }
                }));
            }
        }
    }

    struct Job {
        encrypt: bool,  // true = encrypt, false = decrypt
        state: [u8; 16],
        index: usize,
    }


    pub fn encrypt_file<P: AsRef<Path>>(read_from: P, write_to: P, key: &[u8; 16], threads: usize) -> io::Result<()> {
        let mut bufs_and_size = prepare_files(read_from, write_to)?;

        // Expand the key
        let expanded_key = key_expansion::expand_key(key);

        // Write encrypted key to start so key can be validated when decrypting.
        let mut key_enc = key.clone();
        aes::encrypt(&mut key_enc, &expanded_key);
        bufs_and_size.write_buf.write(&key_enc)?;

        let mut bytes_done: usize = 0;

        let padding_amount = 16 - (bufs_and_size.file_size % 16);

        let mut child_threads = Vec::with_capacity(threads);

        while bytes_done < bufs_and_size.file_size {
            let mut state = [0u8; 16];
            bufs_and_size.read_buf.read(&mut state)?;

            if bufs_and_size.file_size - bytes_done <= 16 && padding_amount != 0 {
                for i in 0..padding_amount {
                    state[16 - padding_amount + i] = padding_amount as u8;
                }
            }

            child_threads.push(thread::spawn(move || {
                let mut state = state;  // Claim it
                aes::encrypt(&mut state, &expanded_key);
                (state, bytes_done)
            }));

            bytes_done += 16;
            println!("Bytes sent: {}", bytes_done);
        }

        println!("Getting results");

        let mut results: Vec<([u8; 16], usize)> = child_threads
            .into_iter()
            .map(|child| child.join().unwrap())
            .collect();

        results.sort_by(|(_, index_a), (_, index_b)| index_a.cmp(index_b));

        for result in results {
            bufs_and_size.write_buf.write(&result.0)?;
        }

        bufs_and_size.write_buf.flush()?;   // Write remaining stuff

        Ok(())
    }
}
*/

/*  Padding is added at the end of the file so it can be split into 16 byte chunks.
    For example, if 4 more bytes are needed for padding, then four "4"s are added to the end.
    e.g [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 ,12] -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 4, 4, 4, 4]
    Then when decrypting, four "4"s are found in a row in the last block, so are removed.
*/
pub fn encrypt_file<P: AsRef<Path>>(read_from: P, write_to: P, key: &[u8; 16]) -> io::Result<()> {
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
        bufs_and_size.read_buf.read(&mut state)?;

        if padding_amount != 0 && bufs_and_size.file_size - bytes_done <= 16 {
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

pub fn decrypt_file(read_from: &Path, write_to: &Path, key: &[u8; 16]) -> io::Result<()> {
    println!("Decrypting.");
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
        bufs_and_size.write_buf.write(&state)?;
        bytes_done += 16;
    }

    // Last block may have padding
    bufs_and_size.read_buf.read(&mut state)?;
    aes::decrypt(&mut state, &expanded_key);

    // Check for padding by checking for consecutive of the same end value
    let end_val = state[15] as usize;
    let mut end_chunk = state.to_vec();
    if end_val <= 16 && end_val > 0 {
        if end_chunk[(16-end_val)..16].to_vec() == vec![end_val as u8; end_val] {
            end_chunk.truncate(16-end_val);
        }
    }

    bufs_and_size.write_buf.write(end_chunk.as_slice())?;
    bufs_and_size.write_buf.flush()?;   // Write remaining stuff

    Ok(())
}


struct BuffersAndSize {
    write_buf: BufWriter<File>,
    read_buf: BufReader<File>,
    file_size: usize,
}

fn prepare_files<P: AsRef<Path>>(read_from: P, write_to: P) -> io::Result<BuffersAndSize> {
    let read_file = File::open(read_from)?;
    let size = read_file.metadata().unwrap().len() as usize;

    let write_file = File::create(write_to)?;

    Ok(BuffersAndSize {
        write_buf: BufWriter::with_capacity(BUFFER_SIZE, write_file),
        read_buf: BufReader::with_capacity(BUFFER_SIZE, read_file),
        file_size: size,
    })
}


// #[cfg(test)]
// mod tests {
//     use test::Bencher;
//     use std::path::Path;

//     #[bench]
//     fn encrypt_file(b: &mut Bencher) {
//         b.iter(|| super::encrypt_file(
//             Path::new("./test_files/julia.png"),
//             Path::new("./test_files/encrypted/julia.enc"),
//             &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
//         ).unwrap());
//     }

//     #[bench]
//     fn encrypt_large_file(b: &mut Bencher) {
//         b.iter(|| super::encrypt_file(
//             Path::new("./test_files/city.jpg"),
//             Path::new("./test_files/encrypted/city.enc"),
//             &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
//         ).unwrap());    
//     }

//     #[bench]
//     fn decrypt_file(b: &mut Bencher) {
//         b.iter(|| super::decrypt_file(
//             Path::new("./test_files/encrypted/julia.enc"),
//             Path::new("./test_files/decrypted/julia.png"),
//             &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
//         ).unwrap());
//     }

//     #[bench]
//     fn decrypt_large_file(b: &mut Bencher) {
//         b.iter(|| super::encrypt_file(
//             Path::new("./test_files/encrypted/city.enc"),
//             Path::new("./test_files/decrypted/city.jpg"),
//             &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
//         ).unwrap());    
//     }
// }
