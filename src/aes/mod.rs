mod lookup_tables;
pub mod key_expansion;

#[inline]
fn add_round_key(state: &mut [u8], round_key: &[u8]) {
    for i in 0..16 {
        state[i] ^= round_key[i];
    }
}

#[inline]
fn sub_bytes(state: &mut [u8]) {
    use lookup_tables::SBOX;
    for i in 0..16 {
        state[i] = SBOX[state[i] as usize];
    }
}

#[inline]
fn shift_rows(state: &mut [u8]) {
    let mut temp = [0u8; 16];

    temp[ 0] = state[ 0];
    temp[ 1] = state[ 5];
    temp[ 2] = state[10];
    temp[ 3] = state[15];

    temp[ 4] = state[ 4];
    temp[ 5] = state[ 9];
    temp[ 6] = state[14];
    temp[ 7] = state[ 3];

    temp[ 8] = state[ 8];
    temp[ 9] = state[13];
    temp[10] = state[ 2];
    temp[11] = state[ 7];

    temp[12] = state[12];
    temp[13] = state[ 1];
    temp[14] = state[ 6];
    temp[15] = state[11];

    for i in 0..16 {
        state[i] = temp[i];
    }
}

#[inline]
fn mix_columns(state: &mut [u8]) {
    use lookup_tables::{MUL_2, MUL_3};

    let mut temp = [0u8; 16];

    temp[ 0] = MUL_2[state[ 0] as usize] ^ MUL_3[state[ 1] as usize] ^ state[ 2] ^ state[ 3];
    temp[ 1] = state[ 0] ^ MUL_2[state[ 1] as usize] ^ MUL_3[state[ 2] as usize] ^ state[ 3];
    temp[ 2] = state[ 0] ^ state[ 1] ^ MUL_2[state[ 2] as usize] ^ MUL_3[state[ 3] as usize];
    temp[ 3] = MUL_3[state[ 0] as usize] ^ state[ 1] ^ state[ 2] ^ MUL_2[state[ 3] as usize];

    temp[ 4] = MUL_2[state[ 4] as usize] ^ MUL_3[state[ 5] as usize] ^ state[ 6] ^ state[ 7];
    temp[ 5] = state[ 4] ^ MUL_2[state[ 5] as usize] ^ MUL_3[state[ 6] as usize] ^ state[ 7];
    temp[ 6] = state[ 4] ^ state[ 5] ^ MUL_2[state[ 6] as usize] ^ MUL_3[state[ 7] as usize];
    temp[ 7] = MUL_3[state[ 4] as usize] ^ state[ 5] ^ state[ 6] ^ MUL_2[state[ 7] as usize];

    temp[ 8] = MUL_2[state[ 8] as usize] ^ MUL_3[state[ 9] as usize] ^ state[10] ^ state[11];
    temp[ 9] = state[ 8] ^ MUL_2[state[ 9] as usize] ^ MUL_3[state[10] as usize] ^ state[11];
    temp[10] = state[ 8] ^ state[ 9] ^ MUL_2[state[10] as usize] ^ MUL_3[state[11] as usize];
    temp[11] = MUL_3[state[ 8] as usize] ^ state[ 9] ^ state[10] ^ MUL_2[state[11] as usize];

    temp[12] = MUL_2[state[12] as usize] ^ MUL_3[state[13] as usize] ^ state[14] ^ state[15];
    temp[13] = state[12] ^ MUL_2[state[13] as usize] ^ MUL_3[state[14] as usize] ^ state[15];
    temp[14] = state[12] ^ state[13] ^ MUL_2[state[14] as usize] ^ MUL_3[state[15] as usize];
    temp[15] = MUL_3[state[12] as usize] ^ state[13] ^ state[14] ^ MUL_2[state[15] as usize];

    for i in 0..16 {
        state[i] = temp[i];
    }
}

#[inline]
pub fn encrypt(state: &mut [u8], expanded_key: &[u8; 176]) {
    add_round_key(state, &expanded_key[..16]);

    for i in (16..160).step_by(16) {     // 9 * 16 =  144, + 16 since first block is key
        sub_bytes(state);
        shift_rows(state);
        mix_columns(state);
        add_round_key(state, &expanded_key[i..i+16]);
    }

    sub_bytes(state);
    shift_rows(state);
    add_round_key(state, &expanded_key[160..]);
}



#[cfg(test)]
mod tests {
    use test::Bencher;

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

    #[bench]
    fn encrypt(b: &mut Bencher) {
        b.iter(|| super::encrypt(&mut [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], &EXPANDED_KEY));
    }
}