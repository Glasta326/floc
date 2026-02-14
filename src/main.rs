mod config;

fn main() -> Result<(), String> {
    let cfg = config::Config::parse_config()?;

    println!("Input: {:?}", cfg.fp_in);
    match cfg.fp_out {
        Some(x) => println!("Output: {:?}", x),
        None => println!("No output specified"),
    }
    println!("Chunksize: {}", cfg.chunk_size);

    let mut x = 0;

    Ok(())
}
