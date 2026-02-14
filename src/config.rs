use std::{
    env::{Args, args},
    path::{Path, PathBuf},
    process::exit,
};

// Program configuration struct
pub struct Config {
    pub fp_in: PathBuf,          // Location of the input file.
    pub fp_out: Option<PathBuf>, // Optional location of output file. If None then same as input file's folder
    pub chunk_size: usize,       // How large each data chunk read from the input file is
    pub no_max: bool,
}

impl Config {
    // Reads program arguments and attempts to create a config struct to define user parameters
    // Returns a Config struct when OK
    // Returns an error string when ERR
    pub fn parse_config() -> Result<Config, String> {
        // Collect args into a String vec because it is easier to work with for special argument types
        let mut argstr: Vec<String> = std::env::args().map(|a| a.to_lowercase()).collect();

        let mut in_file: PathBuf = PathBuf::new();
        let mut out_file: Option<PathBuf> = None;
        let mut chunk_size: usize = 1024;
        let mut no_max = false;

        // If the user provides no arguments at all or puts the help argument anywhere then print help text and exit
        if argstr.iter().any(|a| a == "-h" || a == "--help") {
            Self::print_help();
            exit(0);
        }

        if argstr.iter().any(|a| a == "-v" || a == "--version") {
            println!("{}: {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            exit(0);
        }

        // Skip the first arg as it is always an auto-generated argument and we dont need it
        let mut args = std::env::args_os().skip(1);

        // first parameter is the target file
        let targetFile = args.next().ok_or("Please specify a target file")?;
        in_file = PathBuf::from(targetFile);

        // Optional arguments come in any order and any amount and so are handled last and dynamically
        while let Some(arg) = args.next() {
            match arg.to_string_lossy().to_lowercase().as_ref() {
                "--chunksize" => {
                    let val = args
                        .next()
                        .ok_or("--chunksize requires a value parameter")?;
                    let res = val.to_string_lossy().parse::<usize>();

                    match res {
                        Ok(x) => chunk_size = x,
                        Err(x) => return Err(format!("{}", x)),
                    }
                }

                "--output" | "-o" => {
                    let val = args
                        .next()
                        .ok_or("--output requires a filename or filepath")?;
                    out_file = Some(PathBuf::from(val));
                }

                "-nm" | "--nomax" => {
                    no_max = true;
                }

                _ => return Err(format!("Unknown parameter: '{:?}'", arg)),
            }
        }

        return Ok(Config {
            fp_in: in_file,
            fp_out: out_file,
            chunk_size,
            no_max
        });
    }

    // Prints the help text displaying all the command information
    fn print_help() {
        println!(
            "
        Usage: encrypt [FILE] [OPTIONS]
        
        Options:
            [-v | --version]: Display the current application version
            [-h | --help]: Display this help text
            [-o <value> | --output<value>]: Sets the output file name. Default value is the same as the input file's name.
            [--chunksize <value>]: Sets the file chunking size to <value> number of bytes. Default value is 1024.
            [-nm | --nomax]: Disables the iteration safety limit. Only relevant for very large files.

        Example:
            encrypt passwords.txt -o secret --chunksize 2048
        "
        );
    }
}
