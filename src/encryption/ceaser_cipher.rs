use crate::config_gen::Config;

// TODO: real secure algorithm like RSA
pub fn encrypt_data_chunk(data: &[u8], cfg: &Config) -> Vec<u8> {
    println!("Encrypting {} bytes...", &data.len());

    let mut output: Vec<u8> = Vec::new();

    for byte in data {


        let mut thisb = *byte;
        if thisb == 255 {
            thisb = 0;
        }
        else {
            thisb += 1;
        }
        output.push(thisb);
    }

    return output;
}

pub fn decrypt_data_chunk(data: &[u8], cfg: &Config) -> Vec<u8> {
    println!("Decrypting {} bytes... ", &data.len());

    let mut output: Vec<u8> = Vec::new();

    for byte in data {
        let mut thisb = *byte;

        if thisb == 0{
            thisb = 255;
        }
        else {
            thisb -= 1;
        }
        output.push(thisb);
    }

    return output;
}
