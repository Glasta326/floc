use std::{
    fs::File,
    io::{BufReader, Read},
    os::unix::fs::FileExt,
};

mod config;

fn main() -> Result<(), String> {
    let program_cfg = config::Config::parse_config()?;

    // Display parsed config
    println!("Input: {:?}", program_cfg.fp_in);
    match program_cfg.fp_out {
        Some(x) => println!("Output: {:?}", x),
        None => println!("No output specified"),
    }
    println!("Chunksize: {}", program_cfg.chunk_size);
    println!("No max:{}", program_cfg.no_max);

    let mut working_file = match File::open(&program_cfg.fp_in) {
        Ok(f) => {
            println!("File found.");
            f
        }
        Err(_) => {
            return Err(format!(
                "Could not find file: {}.",
                &program_cfg.fp_in.to_string_lossy()
            ));
        } // Convert it to a string so we dont have to use debug formatting
    };

    let mut data_buffer: Vec<u8> = vec![0u8; program_cfg.chunk_size];
    let mut reader = BufReader::new(working_file);
    let mut iter_count = 0;

    // With "no limits", we use usize max just as a final safety net still
    // 1_000_000 default limit with default chunk size of 1kb means default limit is a 1gb file which is reasonable
    let iter_limit = {
        if program_cfg.no_max {
            usize::MAX
        } else {
            1_000_000
        }
    };

    // TODO: make this inf loop
    while iter_count < iter_limit {
        // When there are 0 bytes left to read that means EOF and we should exit
        let bytes_read = reader.read(&mut data_buffer).unwrap();
        if bytes_read == 0 {
            break;
        }

        let this_data = &data_buffer[0..bytes_read]; // can have this as function input: handle_data(&data_buffer[0..bytes_read])
        println!("bytes to process: {}", &this_data.len());

        iter_count += 1;
        println!("Iteration: {}", iter_count)
    }

    Ok(())
}
