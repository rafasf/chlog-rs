# chlog-rs

Playing with Rust

## Usage

```sh
USAGE:
    chlog-rs [OPTIONS] --repository <repository path>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <changelog output file name>    The name of the file to be created
    -n, --range <initial-hash..final-hash>     Range of commits to include (using Git style from..to)
    -r, --repository <repository path>         The path to the repository
```

Example

`$ chlog-rs -r ./path-to-repo/.git --file changelog.md --range 06a3ba7..9cea8b4`

## TODO

* [ ] Create Makefile for packaging & eventually publishing
* [ ] Externalise user configuration
* [ ] Externalise application configuration (and find out how to package it)
* [ ] Create trait for changelog writers
* [ ] Be smarter on printing/logging

