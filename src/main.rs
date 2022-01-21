mod days;
mod util;

fn main() -> Result<(), &'static str> {
    let res = days::day4::part2();

    println!("Result: {:?}", res);

    Ok(())
}
