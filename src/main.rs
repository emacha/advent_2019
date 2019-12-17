use advent_2019::*;
use std::io::{Result, BufReader, BufRead};
use std::fs;

fn main() -> Result<()> {
    let total = (183565..657474).fold(0, |acc, x| acc + (passes_rules(&x)) as i32);

    println!("{:?}", total);
    Ok(())
}
