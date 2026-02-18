use std::{
    env::{Args, args},
    path::{Path, PathBuf},
    process::exit,
};

/// Program configuration struct
/// Holds data about all the settings the user ran this program with, as well as useful context like the target file's name, extension and ect
pub struct Config {
    /// Toggles whether the program ENCRYPTS (True) or DECRYPTS (false).
    pub encryption_mode: bool,

    /// Location of the input file
    pub fp_in: PathBuf,

    /// (Optional) location of the output file. If None then program will use the same folder as input file
    pub fp_out: Option<PathBuf>,

    /// The name of the input file, without extension
    pub in_file_name: String,

    /// The input file's extension, if there is one (.txt,.png,ect)
    pub in_file_ext: Option<String>,

    /// The name of the output file, without extension
    pub out_file_name: Option<String>,

    /// The output file's extension, if there is one (.txt,.png,ect)
    pub out_file_ext: Option<String>,

    /// How large each data chunk read from the input file is
    pub chunk_size: usize,

    /// Controls the size limit safety check incase the program gets stuck in an endless loop
    pub no_max: bool,
}

impl Config {
    // Reads program arguments and attempts to create a config struct to define user parameters
    // Returns a Config struct when OK
    // Returns an error string when ERR
    pub fn parse_config() -> Result<Config, String> {
        // Collect args into a String vec because it is easier to work with for special argument types
        let mut argstr: Vec<String> = std::env::args().map(|a| a.to_lowercase()).collect();

        let mut encryption_mode: bool = true;
        let mut in_file: PathBuf = PathBuf::new();
        let mut out_file: Option<PathBuf> = None;
        let mut in_file_name: String = String::new();
        let mut in_file_ext: Option<String> = None;
        let mut out_file_name: Option<String> = None;
        let mut out_file_ext: Option<String> = None;
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

        // Get the program's operating mode
        let mode = args
            .next()
            .ok_or("Please provide a program operation mode")?
            .to_string_lossy()
            .into_owned();
        match mode.as_ref() {
            "encrypt" | "e" => {
                encryption_mode = true;
            }
            "decrypt" | "d" => {
                encryption_mode = false;
            }

            _ => {
                return Err(format!(
                    "Please provide a valid operation mode! See help for details (use --help or -h)"
                ));
            }
        }

        // first parameter is the target file
        let targetFile = args.next().ok_or("Please specify a target file")?;
        in_file = PathBuf::from(&targetFile);

        in_file_name = in_file
            .file_stem()
            .ok_or("Invalid filename")?
            .to_string_lossy()
            .into_owned();

        in_file_ext = in_file
            .extension()
            .map(|ext| ext.to_string_lossy().into_owned());

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

                    // TODO: .clone().unwrap() are you fucking kidding me
                    out_file_name = out_file.clone().unwrap()
                        .file_stem()
                        .map(|ext| ext.to_string_lossy().into_owned());

                    out_file_ext = out_file.clone().unwrap()
                        .extension()
                        .map(|ext| ext.to_string_lossy().into_owned());
                }

                "-nm" | "--nomax" => {
                    no_max = true;
                }

                _ => return Err(format!("Unknown parameter: '{:?}'", arg)),
            }
        }

        return Ok(Config {
            encryption_mode,
            fp_in: in_file,
            fp_out: out_file,
            in_file_name,
            in_file_ext,
            out_file_ext,
            out_file_name, // TODO: make these x_file_ext and _name's into impl functions that just extract from the fp_in/fp_out
            chunk_size,
            no_max,
        });
    }

    // Prints the help text displaying all the command information
    fn print_help() {
        println!(
            "
        Usage: rsa [MODE] [FILE] (OPTIONS)

        Modes:
            [encrypt | e]: Encrypts a target file into a .rsa file
            [decrypt | d]: Decrypts a target file of any type
        
        Options:
            [-v | --version]: Display the current application version
            [-h | --help]: Display this help text
            [-o <value> | --output<value>]: Sets the output file name:
                If used on decryption this will override the original file's name.
                If a new extension is provided too this will also override the original file's extension.
                Default value is the same as the input file's name.
            [--chunksize <value>]: Sets the file chunking size to <value> number of bytes. Default value is 1024.
            [-nm | --nomax]: Disables the iteration safety limit. Only relevant for very large files.

        Examples:
            rsa encrypt passwords.txt -o secret --chunksize 2048
            rsa d data.rsa -nm -o secret

        Detailed example:
            > ls
            mysecretfile.rsa
            > rsa d mysecretfile.png -o decrypted -nm
            > ls
            mysecretfile.rsa    decrypted.txt
        "
        );
    }

    // Get the ending file extension of this file
    // Returns a string with the found file type if one exists
    // Returns None if the file has no extension
    // Example: "thisisafile.txt.png" would return ".png"

    /// Returns the trailing file extension on the input string, which is assumed to be a file name
    ///
    /// # Arguments
    ///
    /// * `file_name` - The target string to search through
    ///     
    /// # Returns
    ///
    /// * [`None`] - if no extension could be found
    /// * [`String`] - containing the found extension
    ///
    /// # Examples
    /// ```
    /// let file_name = "something.txt".to_String();
    /// let file_nameless = "something".to_String();
    /// let file_ext = get_file_ext(&file_name);
    /// let file_extless = get_file_ext(&file_nameless);
    ///
    /// assert_eq!(file_ext, String::from(".txt"));
    /// assert_eq!(file_extless, None);
    /// ```
    fn get_file_ext(file_name: &String) -> Option<String> {
        return None;
        /* 
        // Find the index of the trailing '.'
        let index = Self::scan_backwards(file_name, '.')?;

        let mut split = file_name.clone();

        // This returns the part split to the right, which we want
        return Some(split.split_off(index));
        */
    }


}
