use crate::aes::lookup_tables::{SBOX, R_CON};

pub fn expand_key(input_key: &[u8; 16]) -> [u8; 176] {
    let mut expanded = [0u8; 176];
    expanded[..16].clone_from_slice(&input_key[..]);

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

        for num in temp.iter() {
            expanded[bytes_generated] = expanded[bytes_generated-16] ^ num;
            bytes_generated += 1;
        }
    }

    expanded
}

#[cfg(test)]
mod tests {
    use super::*;
    // use test::Bencher;

    // #[bench]
    // fn expand_key(b: &mut Bencher) {
    //     b.iter(|| expand_key(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]));
    // }

    const TEST_KEY: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

    // Expanded sequence of 1 to 16
    const EXPANDED_KEY: [u8; 176] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        171, 116, 201, 211, 174, 114, 206, 219, 167, 120, 197,
        215, 170, 118, 202, 199, 145, 0, 15, 127, 63, 114, 193,
        164, 152, 10, 4, 115, 50, 124, 206, 180, 133, 139, 130,
        92, 186, 249, 67, 248, 34, 243, 71, 139, 16, 143, 137, 63,
        254, 44, 247, 150, 68, 213, 180, 110, 102, 38, 243, 229,
        118, 169, 122, 218, 61, 246, 160, 174, 121, 35, 20, 192,
        31, 5, 231, 37, 105, 172, 157, 255, 140, 168, 182, 87, 245,
        139, 162, 151, 234, 142, 69, 178, 131, 34, 216, 77, 95, 201,
        85, 187, 170, 66, 247, 44, 64, 204, 178, 158, 195, 238, 106,
        211, 247, 203, 51, 149, 93, 137, 196, 185, 29, 69, 118, 39,
        222, 171, 28, 244, 142, 87, 140, 136, 211, 222, 72, 49, 206,
        155, 62, 22, 16, 48, 34, 226, 188, 196, 20, 66, 111, 26, 92,
        115, 161, 129, 98, 101, 177, 177, 64, 135
    ];

    #[test]
    fn expand_key_test() {
        let expanded = expand_key(&TEST_KEY);

        for i in 0..expanded.len() {
            assert_eq!(expanded[i], EXPANDED_KEY[i]);
        }
    }
}