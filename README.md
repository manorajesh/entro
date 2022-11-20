# entro
Originally designed to determine the entropy of a file, this Rust program counts the character frequency from a file or `stdin`.

## Installation
`cargo install entro`

## Usage
```
Program to count character frequency from FILE or STDIN

Usage: entro [OPTIONS] [FILE]

Arguments:
  [FILE]  Input file or STDIN if not provided

Options:
  -x, --hex                   Print in hex
  -b, --binary                Print in binary
  -l, --length <CHARACTERS>   Length of bars in histogram [default: 50]
      --bar-char <CHARACTER>  Bar character [default: ─]
      --bar-cap <CHARACTER>   Bar cap character (ending) [default: ⎸]
  -h, --help                  Print help information
  -V, --version               Print version information
```
<br>The `-x` flag prints the key for the character in hexadecimal
<br>The `-b` option prints the key for the character in binary
<br>The `-l` option modifies the maximum length of the histogram (all data is clamped within this value)
<br>The `--bar-char` option changes what character is used for the bar portion of the histogram
<br>The `--bar-cap` option changes what character is used for the ending character for the bar (the cap)
