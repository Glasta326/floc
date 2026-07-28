# floc

A simple CLI tool for encrypting and decrypting files built without external cryptography libraries

As of 06/03/2026, this project is still unfinished and should not be used for security

## Overview

**Features**

 - Chunk-based processing so arbitrary sized files can be processed while keeping RAM usage light
 - encrypted file metadata, so decryption restores the original file's name and filetype
 - option to override any decryption metadata and specify file output 

**Future TODOs**

 - Further security enhancements: Salting, padding, ect..
 - Support for providing a target key to ecrypt with so files can be sent to other users via CLI
 - Multithreaded data loading and chunking (Noticable wait times to retrieve data when using larger chunksizes)

## Installation

For now, clone the repo and build from source:

```bash
git clone https://github.com/Glasta326/floc.git
cd floc
cargo build --release
```

And optionally, installed with
```bash
cargo install --path .
```

## Usage

### Encrypt a file

```bash
floc e myfile.ext
```

### decrypt a file and override the contained filename metadata

```bash
floc d ecryptedfile.rsa -o output.png
```

## Extra usage info / CLI options

```bash
[-v | --version]: Display the current application version
[-h | --help]: Display this help text
[-o <value> | --output<value>]: Sets the output file name:
    If used on decryption this will override the original file's name.
    If a new extension is provided too this will also override the original file's extension.
    Default value is the same as the input file's name.
[--chunksize <value>]: Sets the file chunking size to <value> number of bytes. Default value is 1024.
[-nm | --nomax]: Disables the iteration safety limit. Only relevant for very large files.
[-k<value> | --key <value>]: provide an external base64 key to proccess this file with.
```
