/*A CLI that generates random fruits */

use calc_cli_with_tests::{get_fruits, is_portuguese_fruit};  // Import the functions from your library
use clap::Parser;

/// CLI tool to return random fruits
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The quantity of fruits to return
    #[clap(short, long, default_value = "1")]
    count: u32,
    /// The fruit to check if it's native to Portugal
    #[clap(short, long)]
    check_fruit: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(fruit) = &args.check_fruit {
        // Check if the provided fruit is native to Portugal
        if is_portuguese_fruit(fruit) {
            println!("{} is a native Portuguese fruit.", fruit);
        } else {
            println!("{} is not native to Portugal or the Azores.", fruit);
        }
    } else {
        // Generate and print random fruits
        let fruits = get_fruits(args.count);
        println!("Fruits: {:?}", fruits);
    }
}
