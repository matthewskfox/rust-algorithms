use clap::Parser;
use std::io;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_parser = clap::value_parser!(u16).range(2015..=2025))]
    year: u16,

    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,

    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: Option<u8>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    println!("Running Year {} - Day {}.{}", args.year, args.day, args.part.unwrap_or(1));

    let result = match (args.year, args.day, args.part) {
        (2025, 1, Some(1)) => adventofcode::year2025::day01::solve1()?,

        _ => {
            eprintln!("Error: Solution for Year {} Day {}.{} is not implemented yet.", args.year, args.day, args.part.unwrap_or(1));
            return Ok(());
        }
    };

    println!("Answer: {}", result);

    Ok(())
}