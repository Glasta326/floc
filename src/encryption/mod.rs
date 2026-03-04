use std::{ops::Mul, str::FromStr};

use num_bigint::BigUint;

pub mod ceaser_cipher;
pub mod process_file;
pub mod rsa;

pub struct key_data {
    pub p: BigUint,
    pub q: BigUint,
    pub n: BigUint,
    pub phi: BigUint,
    pub e: BigUint,
    pub d: BigUint,
}

impl key_data {
    pub fn new(p: BigUint, q: BigUint) -> Self {
        let n = &p * &q;
        let phi = (&p - 1u8) * (&q - 1u8);
        let e = BigUint::from(65537 as u32);
        let d = BigUint::modinv(&e, &phi).unwrap();
        return key_data {
            p: (p),
            q: (q),
            n: (n),
            phi: (phi),
            e: (e),
            d: (d),
        };
    }

    
}

// ecryption: ((message) ^ e) * mod n
// ecryption: ((ciphertext) ^ d) * mod n
