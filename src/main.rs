use advent_2019::fuel_cost;
use std::io::{BufReader, Result, BufRead};
use std::fs::File;


fn main() -> Result<()> {
    let input = File::open("inputs/day_1.txt")?;
    let input = BufReader::new(input);

    let mut fuel_sum = 0;
    for module_mass in input.lines() {
        let value: i64 = module_mass.unwrap().parse().unwrap();
        fuel_sum += fuel_cost(value)
    }

    println!("{}", fuel_sum);
    Ok(())
}
