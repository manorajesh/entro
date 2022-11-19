# entro
Originally designed to determine the entropy of a file, this Rust program counts the character frequency from `stdin`

## Installation
`cargo install entro`

## Usage
```
Program to count character frequency from STDIN

Usage: entro [OPTIONS]

Options:
  -x, --hex              Print in hex
  -l, --length <LENGTH>  Maximum number of characters to print [default: 50]
  -h, --help             Print help information
  -V, --version          Print version information
```
<br>The `-x` flag prints the key for the character in hexadecimal
<br>The `-l` option modifies the maximum length of the histogram (all data is clamped within this value)
