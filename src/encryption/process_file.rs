use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};

use crate::{config_gen::Config, encryption};

// Takes data chunks from the input file and performs the encryption on them, and then writes them to the output file.
pub fn encrypt_file(in_file: &File, out_file: &mut File, cfg: &Config) -> Result<(), String> {
    let mut data_buffer: Vec<u8> = vec![0u8; cfg.chunk_size];
    let mut reader = BufReader::new(in_file);
    let mut iter_count = 0;

    // With "no limits", we use usize max just as a final safety net still
    // 1_000_000 default limit with default chunk size of 1kb means default limit is a 1gb file which is reasonable
    let iter_limit = { if cfg.no_max { usize::MAX } else { 1_000_000 } };

    // First write in the metadata chunk, which is just file name and extension:

    let mut metadata: Vec<u8> = generate_metadata(&cfg);
    let metadata_bytes = write_to_output(out_file, &metadata)?;
    println!("Wrote {} bytes of metadata", metadata_bytes);

    while iter_count < iter_limit {
        // When there are 0 bytes left to read that means EOF and we should exit
        let bytes_read = reader.read(&mut data_buffer).unwrap();
        if bytes_read == 0 {
            break;
        }

        let processed_data =
            encryption::ceaser_cipher::encrypt_data_chunk(&data_buffer[0..bytes_read], cfg);

        let bytes_written = write_to_output(out_file, &processed_data)?;
        println!("{} bytes written to file", bytes_written);

        // Ensure at minimum we are writing the same number of bytes as we read
        assert!(bytes_written <= bytes_read, "Data was lost!");

        iter_count += 1;
        println!("Iteration: {}", iter_count)
    }

    return Ok(());
}

// File metadata is essentially just [has_ext][name bytes][null][ext bytes?][null?][data...]
pub fn generate_metadata(cfg: &Config) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    // Write whether this file has an extension
    output.push(cfg.encryption_mode as u8);

    // write name
    output.extend_from_slice(cfg.file_name.as_bytes());
    output.push(0x00);

    // write cfg if applicable
    match &cfg.file_ext {
        Some(v) => {
            output.extend_from_slice(v.as_bytes());
            output.push(0x00);
        }
        None => {}
    }

    return output;
}

pub fn write_to_output(out_file: &mut File, data: &[u8]) -> Result<usize, String> {
    let res = out_file.write(data);

    // There's probaly some .SolveEverything() extension that does this but whatever no harm in doing it manually compiler can optimise this
    match res {
        Ok(x) => return Ok(x),
        Err(x) => return Err(x.to_string()),
    }
}
