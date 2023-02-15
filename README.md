# Advent of Code

Solution to some of the AoC challanges. Written in Rust, trying to use iterators where possible.

Includes Python script to download inputs from the official website and generate corresponding Rust module.

## Usage

Setup script:
```shell
day_setup.py [year] [day]
```

AoC challange:
```shell
cargo run --release -- --year <YEAR> --day <DAY> --part <PART>
```

## Example
```shell
day_setup.py 22 1
cargo run --release -- --year 2022  --day 1 --part 1
```
