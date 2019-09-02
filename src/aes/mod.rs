mod lookup_tables;
pub mod key_expansion;

macro_rules! add_round_key {($state:expr, $round_key:expr) => {
    *$state = [
        $state[ 0] ^ $round_key[ 0], $state[ 1] ^ $round_key[ 1], $state[ 2] ^ $round_key[ 2], $state[ 3] ^ $round_key[ 3],
        $state[ 4] ^ $round_key[ 4], $state[ 5] ^ $round_key[ 5], $state[ 6] ^ $round_key[ 6], $state[ 7] ^ $round_key[ 7],
        $state[ 8] ^ $round_key[ 8], $state[ 9] ^ $round_key[ 9], $state[10] ^ $round_key[10], $state[11] ^ $round_key[11],
        $state[12] ^ $round_key[12], $state[13] ^ $round_key[13], $state[14] ^ $round_key[14], $state[15] ^ $round_key[15]
    ];
};}

macro_rules! sub_bytes {($state:expr) => {
    use lookup_tables::SBOX;
    *$state = [
        SBOX[$state[ 0] as usize], SBOX[$state[ 1] as usize], SBOX[$state[ 2] as usize], SBOX[$state[ 3] as usize],
        SBOX[$state[ 4] as usize], SBOX[$state[ 5] as usize], SBOX[$state[ 6] as usize], SBOX[$state[ 7] as usize],
        SBOX[$state[ 8] as usize], SBOX[$state[ 9] as usize], SBOX[$state[10] as usize], SBOX[$state[11] as usize],
        SBOX[$state[12] as usize], SBOX[$state[13] as usize], SBOX[$state[14] as usize], SBOX[$state[15] as usize]
    ];
};}

macro_rules! inv_sub_bytes {($state:expr) => {
    use lookup_tables::INV_SBOX;
    *$state = [
        INV_SBOX[$state[ 0] as usize], INV_SBOX[$state[ 1] as usize], INV_SBOX[$state[ 2] as usize], INV_SBOX[$state[ 3] as usize],
        INV_SBOX[$state[ 4] as usize], INV_SBOX[$state[ 5] as usize], INV_SBOX[$state[ 6] as usize], INV_SBOX[$state[ 7] as usize],
        INV_SBOX[$state[ 8] as usize], INV_SBOX[$state[ 9] as usize], INV_SBOX[$state[10] as usize], INV_SBOX[$state[11] as usize],
        INV_SBOX[$state[12] as usize], INV_SBOX[$state[13] as usize], INV_SBOX[$state[14] as usize], INV_SBOX[$state[15] as usize]
    ];
};}

macro_rules! shift_rows {($state:expr) => {
    *$state = [
        $state[ 0], $state[ 5], $state[10], $state[15],
        $state[ 4], $state[ 9], $state[14], $state[ 3],
        $state[ 8], $state[13], $state[ 2], $state[ 7],
        $state[12], $state[ 1], $state[ 6], $state[11]
    ];
};}

macro_rules! inv_shift_rows {($state:expr) => {
    *$state = [
        $state[ 0], $state[13], $state[10], $state[ 7],
        $state[ 4], $state[ 1], $state[14], $state[11],
        $state[ 8], $state[ 5], $state[ 2], $state[15],
        $state[12], $state[ 9], $state[ 6], $state[ 3]
    ];
};}

macro_rules! mix_columns {($state:expr) => {
    use lookup_tables::{MUL_2, MUL_3};
    *$state = [
        MUL_2[$state[ 0] as usize] ^ MUL_3[$state[ 1] as usize] ^ $state[ 2] ^ $state[ 3],
        $state[ 0] ^ MUL_2[$state[ 1] as usize] ^ MUL_3[$state[ 2] as usize] ^ $state[ 3],
        $state[ 0] ^ $state[ 1] ^ MUL_2[$state[ 2] as usize] ^ MUL_3[$state[ 3] as usize],
        MUL_3[$state[ 0] as usize] ^ $state[ 1] ^ $state[ 2] ^ MUL_2[$state[ 3] as usize],

        MUL_2[$state[ 4] as usize] ^ MUL_3[$state[ 5] as usize] ^ $state[ 6] ^ $state[ 7],
        $state[ 4] ^ MUL_2[$state[ 5] as usize] ^ MUL_3[$state[ 6] as usize] ^ $state[ 7],
        $state[ 4] ^ $state[ 5] ^ MUL_2[$state[ 6] as usize] ^ MUL_3[$state[ 7] as usize],
        MUL_3[$state[ 4] as usize] ^ $state[ 5] ^ $state[ 6] ^ MUL_2[$state[ 7] as usize],

        MUL_2[$state[ 8] as usize] ^ MUL_3[$state[ 9] as usize] ^ $state[10] ^ $state[11],
        $state[ 8] ^ MUL_2[$state[ 9] as usize] ^ MUL_3[$state[10] as usize] ^ $state[11],
        $state[ 8] ^ $state[ 9] ^ MUL_2[$state[10] as usize] ^ MUL_3[$state[11] as usize],
        MUL_3[$state[ 8] as usize] ^ $state[ 9] ^ $state[10] ^ MUL_2[$state[11] as usize],

        MUL_2[$state[12] as usize] ^ MUL_3[$state[13] as usize] ^ $state[14] ^ $state[15],
        $state[12] ^ MUL_2[$state[13] as usize] ^ MUL_3[$state[14] as usize] ^ $state[15],
        $state[12] ^ $state[13] ^ MUL_2[$state[14] as usize] ^ MUL_3[$state[15] as usize],
        MUL_3[$state[12] as usize] ^ $state[13] ^ $state[14] ^ MUL_2[$state[15] as usize]
    ];
};}

macro_rules! inv_mix_columns {($state:expr) => {
    use lookup_tables::{MUL_9, MUL_11, MUL_13, MUL_14};
    *$state = [
        MUL_14[$state[ 0] as usize] ^ MUL_11[$state[ 1] as usize] ^ MUL_13[$state[ 2] as usize] ^ MUL_9[$state[ 3] as usize],
        MUL_9[$state[ 0] as usize] ^ MUL_14[$state[ 1] as usize] ^ MUL_11[$state[ 2] as usize] ^ MUL_13[$state[ 3] as usize],
        MUL_13[$state[ 0] as usize] ^ MUL_9[$state[ 1] as usize] ^ MUL_14[$state[ 2] as usize] ^ MUL_11[$state[ 3] as usize],
        MUL_11[$state[ 0] as usize] ^ MUL_13[$state[ 1] as usize] ^ MUL_9[$state[ 2] as usize] ^ MUL_14[$state[ 3] as usize],

        MUL_14[$state[ 4] as usize] ^ MUL_11[$state[ 5] as usize] ^ MUL_13[$state[ 6] as usize] ^ MUL_9[$state[ 7] as usize],
        MUL_9[$state[ 4] as usize] ^ MUL_14[$state[ 5] as usize] ^ MUL_11[$state[ 6] as usize] ^ MUL_13[$state[ 7] as usize],
        MUL_13[$state[ 4] as usize] ^ MUL_9[$state[ 5] as usize] ^ MUL_14[$state[ 6] as usize] ^ MUL_11[$state[ 7] as usize],
        MUL_11[$state[ 4] as usize] ^ MUL_13[$state[ 5] as usize] ^ MUL_9[$state[ 6] as usize] ^ MUL_14[$state[ 7] as usize],

        MUL_14[$state[ 8] as usize] ^ MUL_11[$state[ 9] as usize] ^ MUL_13[$state[10] as usize] ^ MUL_9[$state[11] as usize],
        MUL_9[$state[ 8] as usize] ^ MUL_14[$state[ 9] as usize] ^ MUL_11[$state[10] as usize] ^ MUL_13[$state[11] as usize],
        MUL_13[$state[ 8] as usize] ^ MUL_9[$state[ 9] as usize] ^ MUL_14[$state[10] as usize] ^ MUL_11[$state[11] as usize],
        MUL_11[$state[ 8] as usize] ^ MUL_13[$state[ 9] as usize] ^ MUL_9[$state[10] as usize] ^ MUL_14[$state[11] as usize],

        MUL_14[$state[12] as usize] ^ MUL_11[$state[13] as usize] ^ MUL_13[$state[14] as usize] ^ MUL_9[$state[15] as usize],
        MUL_9[$state[12] as usize] ^ MUL_14[$state[13] as usize] ^ MUL_11[$state[14] as usize] ^ MUL_13[$state[15] as usize],
        MUL_13[$state[12] as usize] ^ MUL_9[$state[13] as usize] ^ MUL_14[$state[14] as usize] ^ MUL_11[$state[15] as usize],
        MUL_11[$state[12] as usize] ^ MUL_13[$state[13] as usize] ^ MUL_9[$state[14] as usize] ^ MUL_14[$state[15] as usize]
    ];
};}

#[inline]
pub fn encrypt(state: &mut [u8; 16], expanded_key: &[u8; 176]) {
    add_round_key!(state, &expanded_key[..16]);

    for i in (16..160).step_by(16) {     // 9 * 16 =  144, + 16 since first block is key
        sub_bytes!(state);
        shift_rows!(state);
        mix_columns!(state);
        add_round_key!(state, &expanded_key[i..i+16]);
    }

    sub_bytes!(state);
    shift_rows!(state);
    add_round_key!(state, &expanded_key[160..]);
}

#[inline]
pub fn decrypt(state: &mut [u8; 16], expanded_key: &[u8; 176]) {
    add_round_key!(state, &expanded_key[160..]);
    inv_shift_rows!(state);
    inv_sub_bytes!(state);

    for i in (16..145).rev().step_by(16) {  // 145 downwards
        add_round_key!(state, &expanded_key[i..i+16]);
        inv_mix_columns!(state);
        inv_shift_rows!(state);
        inv_sub_bytes!(state);
    }

    add_round_key!(state, &expanded_key[..16]);
}


#[cfg(test)]
mod tests {
    use super::*;
    //use test::Bencher;

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

    const TEST_ARR: [u8; 16] = [99, 104, 101, 101, 115, 101, 98, 117, 114, 103, 101, 114, 51, 52, 53, 54];
    // Encrypted using http://aes.online-domain-tools.com/
    const ENCRYPTED_TEST_ARR: [u8; 16] = [0x61, 0x18, 0x80, 0xba, 0x0a, 0xd5, 0xfc, 0x55, 0x81, 0xc9, 0xd9, 0x3a, 0xdf, 0x76, 0x35, 0x7c];

    // #[bench]
    // fn encrypt(b: &mut Bencher) {
    //     // String = 'cheeseburger3456'
    //     b.iter(|| encrypt(&mut TEST_ARR, &EXPANDED_KEY));
    // }

    // #[bench]
    // fn decrypt(b: &mut Bencher) {
    //     b.iter(|| decrypt(&mut TEST_ARR, &EXPANDED_KEY));
    // }

    #[test]
    fn encrypt_test() {
        let mut arr = TEST_ARR;
        encrypt(&mut arr, &EXPANDED_KEY);
        assert_eq!(arr, ENCRYPTED_TEST_ARR);
    }

    #[test]
    fn decrypt_test() {
        let mut arr = ENCRYPTED_TEST_ARR;
        decrypt(&mut arr, &EXPANDED_KEY);
        assert_eq!(arr, TEST_ARR);
    }
}