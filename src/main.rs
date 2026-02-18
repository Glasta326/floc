use std::{
    fs::File,
    io::{BufReader, Read, Write},
    os::unix::fs::FileExt,
};

use crate::config_gen::Config;

mod config_gen;
mod encryption;
mod file_handling;

fn main() -> Result<(), String> {
    let program_cfg = config_gen::Config::parse_config()?;

    // Display parsed config
    println!("Encrypting?: {}", program_cfg.encryption_mode);
    println!("Input: {:?}", program_cfg.fp_in);
    match program_cfg.fp_out {
        Some(ref x) => println!("Output: {:?}.rsa", x),
        None => println!("Output: None"),
    }

    println!("File name: {}", program_cfg.file_name);
    match program_cfg.file_ext {
        Some(ref x) => println!("File extension: {}", x),
        None => println!("File extension: None"),
    }
    println!("Chunksize: {}", program_cfg.chunk_size);
    println!("No max: {}", program_cfg.no_max);
    println!("-----------------------------------------");

    let input_file = file_handling::file_io::get_input_file(&program_cfg)?;
    

    if program_cfg.encryption_mode {
        let mut output_file = file_handling::file_io::create_output_file(&program_cfg)?;
        encryption::process_file::encrypt_file(&input_file, &mut output_file, &program_cfg)?;
    }
    else {
        println!("Not implemented!");
    }

    Ok(())
}
