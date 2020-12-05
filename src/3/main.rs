use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use itertools::Itertools;
use regex::Regex;

fn check(lines: &Vec<String>, num_down: u32, num_across: u32) {
    let width = lines[0].len();

    let mut xval = 0;
    let mut num_hit = 0;

    for l in lines.iter().step_by(num_down as usize) {
        if l.chars().nth(xval % width).unwrap() == '#' {
            num_hit += 1;
        }
        xval += num_across as usize;
    }

    println!("{}", num_hit);
}

fn main() -> std::io::Result<()> {
    let file = File::open("inputs/input3")?;
    let buf_reader = BufReader::new(file);
    let lines: Vec<String> = buf_reader.lines().map(|s| s.unwrap()).collect();

    check(&lines, 1, 1);
    check(&lines, 1, 3);
    check(&lines, 1, 5);
    check(&lines, 1, 7);
    check(&lines, 2, 1);

    Ok(())
}
