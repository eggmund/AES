use crate::aes::lookup_tables::{SBOX, R_CON};

pub fn expand_key(input_key: &[u8; 16]) -> [u8; 176] {
    let mut expanded = [0u8; 176];

    for i in 0..16 {
        expanded[i] = input_key[i];
    }

    let mut bytes_generated: usize = 16;
    let mut rcon_iter: u8 = 1;

    while bytes_generated < 176 {
        let mut temp: [u8; 4] = [
            expanded[bytes_generated-4],
            expanded[bytes_generated-3],
            expanded[bytes_generated-2],
            expanded[bytes_generated-1],
        ];

        if bytes_generated % 16 == 0 {
            // Shift, s-box and XOR round constant with first element
            temp = [
                SBOX[temp[1] as usize] ^ R_CON[rcon_iter as usize],
                SBOX[temp[2] as usize],
                SBOX[temp[3] as usize],
                SBOX[temp[0] as usize]
            ];
            
            rcon_iter += 1;
        }

        for i in 0..4 {
            expanded[bytes_generated] = expanded[bytes_generated-16] ^ temp[i];
            bytes_generated += 1;
        }
    }

    expanded
}

// #[cfg(test)]
// mod tests {
//     use test::Bencher;

//     #[bench]
//     fn expand_key(b: &mut Bencher) {
//         b.iter(|| super::expand_key(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]));
//     }
// }