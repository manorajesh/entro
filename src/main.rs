use std::collections::HashMap;
use clap::Parser;
use std::io::{stdin, Read};
use num::clamp;

#[derive(Parser)]
#[command(
    version = "0.0.1",
    author = "Mano Rajesh",
    about = "Program to count character frequency from STDIN"
)]
struct Args {
    /// Print in hex
    #[arg(short = 'x', long)]
    hex: bool,

    /// Maximum number of characters to print
    #[arg(short = 'l', long, default_value = "50")]
    length: Option<usize>,
}
fn main() {
    let args = Args::parse();

    let print_length = args.length.unwrap_or(50) as i32;
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let freq = frequency(buffer.trim().to_string());

    let max = freq.values().max().unwrap();
    for (key, value) in freq.iter() {
        let count = normalize(*value, *max, print_length) as usize;
        if args.hex {
            print!("{:02x}: {}", *key as u8, "─".repeat(count));
        } else {
            print!("{}:\t{}", key.escape_debug(), "─".repeat(count));
        }
        println!("︳");
    }
}

fn frequency(string: String) -> HashMap<char, i32> {
    let mut frequency = HashMap::new();
    for c in string.chars() {
        let count = frequency.entry(c).or_insert(0);
        *count += 1;
    }
    frequency
}

fn normalize(value: i32, max: i32, new_max: i32) -> i32 {
    clamp((value * new_max) / max, 0, new_max)
}