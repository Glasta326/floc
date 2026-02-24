use std::str::FromStr;

use num_bigint::BigUint;

pub mod ceaser_cipher;
pub mod process_file;
pub mod rsa;

pub struct key_data {
    pub p: BigUint,
    pub q: BigUint,
    pub n: BigUint,
    pub r: BigUint,
    pub e: BigUint,
    pub d: BigUint,
}

impl key_data {
    pub fn new(prime1: BigUint, prime2: BigUint) -> Self {
        let n = &prime1 * &prime2;
        let r = (&prime1 - 1u8) * (&prime2 - 1u8);
        let e = BigUint::from(3 as u32);
        let d = e.modinv(&r).unwrap();
        return key_data {
            p: (prime1),
            q: (prime2),
            n: (n),
            r: (r),
            e: (e),
            d: (d),
        };
    }
}
