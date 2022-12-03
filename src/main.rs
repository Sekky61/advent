mod twentyone;
mod twentytwo;

mod util;

use clap::Parser;

use crate::util::Dispatch;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 2022)]
    year: u64,
    #[arg(short, long)]
    day: u64,
    #[arg(short, long)]
    part: u64,
}

fn main() -> Result<(), &'static str> {
    let args = Args::parse();

    let res = Dispatch::call(args.year, args.day, args.part);

    println!("Result: {:?}", res);

    Ok(())
}
