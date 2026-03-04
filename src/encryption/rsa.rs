use num_bigint::BigUint;

use crate::{config_gen::Config, encryption::key_data};

pub fn encrypt_data_chunk(data: &[u8], keyd: &key_data) -> Vec<u8> {
    println!("Encrypting {} bytes...", &data.len());
    let mut output: Vec<u8> = Vec::new();


    let x = BigUint::from_bytes_le(data);
    let y = x.modpow(&keyd.e, &keyd.n);
    output = y.to_bytes_le();
    
    return output;
}

pub fn decrypt_data_chunk(data: &[u8], keyd: &key_data) -> Vec<u8> {
    println!("Decrypting {} bytes... ", &data.len());
    let mut output: Vec<u8> = Vec::new();


    let x = BigUint::from_bytes_le(data);
    let y = x.modpow(&keyd.d, &keyd.n);
    output = y.to_bytes_le();

    return output;
}


