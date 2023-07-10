# Advent of Code

Solution to some of the AoC challanges. Written in Rust, trying to use iterators where possible.

Includes Python script to download inputs from the official website and generate corresponding Rust module.

## Usage

### Setup script:
Set `AOC_SESSION_ID` environment variable to your session ID (can be found in cookies after logging in to AoC website).
Then, run the script with year and day as arguments:
```shell
day_setup.py [year] [day]
```
This will set up a new Rust module in `src/year/day.rs` and download the input to `inputs/year/day.txt`.

### AoC challange:
To run the solution for a given year, day and part, use:
```shell
cargo run --release -- --year <YEAR> --day <DAY> --part <PART>
```

## Example
```shell
day_setup.py 22 1
cargo run --release -- --year 2022  --day 1 --part 1
```
