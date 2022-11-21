use clap::Parser;
use std::{io::{stdin, Read}, fs::File, collections::HashMap};

#[derive(Parser)]
#[command(
    version = "0.1.1",
    author = "Mano Rajesh",
    about = "Program to count character frequency from FILE or STDIN"
)]
struct Args {
    /// Input file or STDIN if not provided
    file: Option<String>,

    /// Print in hex
    #[arg(short = 'x', long)]
    hex: bool,

    /// Print in binary
    #[arg(short = 'b', long)]
    binary: bool,

    /// Length of bars in histogram
    #[arg(short = 'l', long, default_value = "50", value_name="CHARACTERS")]
    length: Option<usize>,

    /// Bar character
    #[arg(long, default_value = "─", value_name="CHARACTER")]
    bar_char: String,

    /// Bar cap character (ending)
    #[arg(long, default_value = "⎸", value_name="CHARACTER")]
    bar_cap: String
}
fn main() {
    let args = Args::parse();

    let print_length = args.length.unwrap_or(50) as i32;
    let mut buffer = Vec::new();

    if let Some(file) = args.file {
        let mut file = File::open(file).expect("Unable to open file");
        file.read_to_end(&mut buffer).expect("Unable to read file (memory overflow)");
    } else {
        stdin().read_to_end(&mut buffer).expect("Unable to read from STDIN");
    }

    let freq = frequency(buffer);

    let max = freq.values().max().unwrap();
    for (key, value) in freq.iter() {
        let count = normalize(*value, *max, print_length) as usize;
        if args.hex {
            print!("{:02x}: {}", *key as u8, args.bar_char.repeat(count));
        } else if args.binary {
            print!("{:08b}: {}", *key as u8, args.bar_char.repeat(count));
        } else {
            print!("{}:\t{}", (*key as char).escape_debug(), args.bar_char.repeat(count));
        }
        println!("{}", args.bar_cap);
    }
}

fn frequency(chars: Vec<u8>) -> HashMap<u8, i32> {
    let mut frequency = HashMap::new();
    for c in chars {
        let count = frequency.entry(c).or_insert(0);
        *count += 1;
    }
    frequency
}

fn normalize(value: i32, max: i32, new_max: i32) -> i32 {
    clamp((value * new_max) / max, 0, new_max)
}

fn clamp(input: i32, min: i32, max: i32) -> i32 {
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}