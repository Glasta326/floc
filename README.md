# floc

A simple CLI tool for encrypting and decrypting files

## Overview

**Features**

 - Chunk-based processing so arbitrary sized files can be processed while keeping RAM usage light
 - encrypted file metadata, so decryption restores the original file's name and filetype
 - option to override any decryption metadata and specify file output 

**Future TODOs**

 - Proper RSA implementation using keys
 - Support for providing a target key to ecrypt with so files can be sent to other users via CLI
 - Multithreaded data loading and chunking (Noticable wait times to retrieve data when using larger chunksizes)

## Installation

For now, just clone the repo and build from source:

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

