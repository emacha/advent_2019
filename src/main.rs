use advent_2019::*;
use std::io::{BufReader, Result, BufRead, Read};
use std::fs::File;
use std::fs;

#[derive(Debug)]
struct Memory {
    noun: i32,
    verb: i32,
}


fn main() -> Result<()> {
    let code: String = fs::read_to_string("inputs/day_2.txt").unwrap().trim().parse().unwrap();

    let search = || {
        let mut out= Memory {noun: -1, verb: -1};
        for noun in 0..=99 {
            for verb in 0..=99 {
                let new_code = intcode(&new_intcode(&noun.to_string(),
                                                   &verb.to_string(),
                                                   &code));

                if &new_code[0..8] == "19690720" {
                    return Memory {noun, verb};
                }

            }
        }
        out
    };

    let new_code = search();


    println!("{:?}", new_code);
    Ok(())
}
