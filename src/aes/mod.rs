mod lookup_tables;
use lookup_tables::SBOX;

#[inline]
fn add_round_key(state: &mut [u8; 16], round_key: &[u8]) {
    for i in 0..16 {
        state[i] ^= round_key[i];
    }
}

#[inline]
fn sub_bytes(state: &mut [u8; 16]) {
    for i in 0..16 {
        state[i] = SBOX[state[i]];
    }
}

#[inline]
fn shift_rows(state: &mut [u8; 16]) {

}

#[inline]
fn mix_columns(state: &mut [u8; 16]) {

}

#[inline]
fn encrypt(state: &mut [u8; 16], expanded_key: &[u8; 176]) {
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