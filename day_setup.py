# setup for new day

import sys
import types

def year_to_text(year: int):
    texts = ["twenty", "twentyone", "twentytwo", "twentythree", "twentyfour"]
    return texts[year - 20]

def main():
    if len(sys.argv) != 3:
        print("Usage:\tday_setup.py [year] [day]")
        print("Example:\tday_setup.py 22 1")
        return

    year = int(sys.argv[1])
    day = int(sys.argv[2])

    if year > 2000:
        year -= 2000

    if year < 0 or year > 30:
        raise ValueError(f"Year value {year} not valid")
    
    if day < 0 or day > 31:
        raise ValueError(f"Day value {day} not valid")

    yeartext = year_to_text(year)

    with open(f"src/{yeartext}/mod.rs", "a") as mod_file:
        mod_file.write(f"\n#[allow(dead_code)]\npub mod day{day};")

    with open(f"src/{yeartext}/day{day}.rs", "w") as day_file:
        day_file.write(f"""
use crate::util;
use itertools::Itertools;

fn parse_input(lines: Vec<String>) {{
    todo!()
}}

pub fn part1() -> usize {{
    let lines = util::read_lines("inputs/{yeartext}/day{day}").unwrap();
    let x = parse_input(lines);
    //println!("Ans: {{}} {{}}", x, y);
    0
}}
""")

main()