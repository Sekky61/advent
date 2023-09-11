# Advent of Code

```rust
challanges.iter()
          .map(solve)
          .collect::<Vec<_>>()
```

Solution to some of the [AoC](https://adventofcode.com/) challanges.
- Written in Rust, trying to use *iterators* where possible.
- Includes a script to setup a new challange.

## Highlights

| challange | solution |
|-----------|----------|
| [2022/07](https://adventofcode.com/2022/day/7) | [solution](src/twentytwo/day7.rs) |
| [2022/24](https://adventofcode.com/2022/day/24) | [solution](src/twentytwo/day24.rs) |

## Usage

Requires up to date Rust toolchain and Python 3.

### Setup script
Set `AOC_SESSION_ID` environment variable to your session ID (can be found in cookies after logging in to AoC website).
Then, run the script with year and day as arguments:
```shell
day_setup.py [year] [day]
```
This will set up a new Rust module in `src/year/day.rs` and download the input to `inputs/year/day.txt`.
Then, add a line to `impl Dispatch` located in `src/util/mod.rs` (see other lines of the match statement).

### AoC challange
To run the solution for a given year, day and part, use:
```shell
cargo run --release -- --year <YEAR> --day <DAY> --part <PART>
```

## Example
```shell
day_setup.py 22 1
cargo run --release -- --year 2022 --day 1 --part 1
```
