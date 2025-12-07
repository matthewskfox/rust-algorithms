use clap::Parser;
use std::io;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_parser = clap::value_parser!(u16).range(2015..=2025))]
    year: u16,

    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,

    #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: u8,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    println!("Running Year {} - Day {}.{}", args.year, args.day, args.part);

    let result = match (args.year, args.day, args.part) {
        (2025, 1, 1) => adventofcode::year2025::day01::solve1()?.to_string(),
        (2025, 1, 2) => adventofcode::year2025::day01::solve2()?.to_string(),
        (2025, 2, 1) => adventofcode::year2025::day02::solve1()?.to_string(),
        (2025, 2, 2) => adventofcode::year2025::day02::solve2()?.to_string(),

        _ => {
            eprintln!("Error: Solution for Year {} Day {}.{} is not implemented yet.", args.year, args.day, args.part);
            return Ok(());
        }
    };

    println!("Answer: {}", result);

    Ok(())
}