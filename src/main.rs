mod days;
mod util;

fn main() -> Result<(), &'static str> {
    //let res = days::day4::part2();

    let res = days::solve_challenge(1, 1)?;
    println!("Result: {:?}", res);

    Ok(())
}
