use base64::{encode, decode};
use crate::aes::{self, key_expansion};

fn encrypt_string(inp: String, key: &[u8; 16]) -> String {
    
}