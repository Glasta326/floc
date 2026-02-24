use std::{
    fs::File,
    io::{BufReader, Read, Write},
    os::unix::{fs::FileExt, process},
};

use num_bigint::BigUint;

use crate::{config_gen::Config, encryption::rsa};

mod config_gen;
mod encryption;
mod file_handling;

fn main() -> Result<(), String> {
    let x = encryption::key_data::new(BigUint::from(11u32), BigUint::from(3u32));

    

    println!("{}",x.d);

    return Err("()".to_string());
    let mut program_cfg = config_gen::Config::parse_config()?;

    // Display parsed config
    println!("Encrypting?: {}", program_cfg.encryption_mode);
    println!("Input: {:?}", program_cfg.fp_in);
    match program_cfg.fp_out {
        Some(ref x) => println!("Output: {}.rsa", x.to_string_lossy()),
        None => println!("Output: None"),
    }

    println!("File name: {}", program_cfg.in_file_name);
    match program_cfg.in_file_ext {
        Some(ref x) => println!("Input File extension: {}", x),
        None => println!("Input File extension: None"),
    }
    match program_cfg.out_file_name {
        Some(ref x) => println!("Output File name: {}", x),
        None => println!("Output File name: None"),
    }
    match program_cfg.out_file_ext {
        Some(ref x) => println!("Output File extension: {}", x),
        None => println!("Output File extension: None"),
    }
    println!("Chunksize: {}", program_cfg.chunk_size);
    println!("No max: {}", program_cfg.no_max);
    println!("-----------------------------------------");
    let input_file = file_handling::file_io::get_input_file(&program_cfg)?;

    // Glasta326 18/02/2026
    // not super happy with the design here, but good enough for now
    // id be much happier if encryption could have the same formatting and params and decryption, as they both "read stuff from file and write stuff to output file"
    // but because we need to decrypt the file info to even make the output file we're writing to, we have to have this done differently
    // maybe its just not really possible, but more likley a design issue
    if program_cfg.encryption_mode {
        let mut output_file = file_handling::file_io::create_output_file(&program_cfg)?;
        encryption::process_file::encrypt_file(&input_file, &mut output_file, &program_cfg)?;
    } else {
        let mut in_file_reader = BufReader::new(input_file);
        let file_data =
            encryption::process_file::extract_metadata(&mut in_file_reader, &program_cfg)?;

        //let output_file = file_handling::file_io::create_output_file(&program_cfg);
        let mut output_file =
            file_handling::file_io::create_output_file_from_metadata(&program_cfg, &file_data)?;

        encryption::process_file::decrypt_file(
            &mut in_file_reader,
            &mut output_file,
            &program_cfg,
        )?;
    }

    Ok(())
}
