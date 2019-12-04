use advent_2019::*;
use std::io::{Result, BufReader, BufRead};
use std::fs;

#[derive(Debug)]
struct Memory {
    noun: i32,
    verb: i32,
}


fn main() -> Result<()> {
    let input = fs::File::open("inputs/day_3.txt")?;
    let input = BufReader::new(input);
    let mut args: Vec<String> = vec![];
    for line in input.lines() {
        args.push(line.unwrap());
        //if let Ok(x) = line.unwrap() {args.push(x)};
    }

    
    println!("{:?}", intersection_distance(&args[0], &args[1]));
    Ok(())
}
