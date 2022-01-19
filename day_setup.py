# setup for new day

import sys

def main():
    if len(sys.argv) != 2:
        print("No day specified")
        return

    day = sys.argv[1]

    with open("src/days/mod.rs", "a") as mod_file:
        mod_file.write(f"\n#[allow(dead_code)]\npub mod day{day};")

    with open(f"src/days/day{day}.rs", "w") as day_file:
        day_file.write("""
use crate::filereader;
use itertools::Itertools;

fn parse_input(lines: Vec<String>) -> _ {{
    todo!()
}}

pub fn part1() -> usize {{
    let lines = filereader::read_lines("inputs/day{0}").unwrap();
    let x = parse_input(lines);
    //println!("Ans: {{}} {{}}", x, y);
    0
}}""".format(day))

main()