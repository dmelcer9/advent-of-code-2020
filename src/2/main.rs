use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use itertools::Itertools;
use regex::Regex;

fn main() -> std::io::Result<()> {
    let file = File::open("inputs/input2")?;
    let buf_reader = BufReader::new(file);
    let lines: Vec<String> = buf_reader.lines().map(|s| s.unwrap()).collect();

    let mut num = 0;
    let pattern = Regex::new(r"(\d+)-(\d+) ([a-zA-Z]): ([a-zA-Z]+)").unwrap();

    for l in lines {
        let caps = pattern.captures(&l).unwrap();

        let min : usize = caps[1].parse().unwrap();
        let max : usize = caps[2].parse().unwrap();
        let pat  = &caps[3].chars().next().unwrap();
        let tosearch = &caps[4];

        if let Some(charAtFirst) = tosearch.chars().nth(min - 1) {
            if let Some(charAtLast) = tosearch.chars().nth(max - 1) {
                if (charAtFirst == *pat) ^ (charAtLast == *pat) {
                    num += 1;
                }
            }
        }

    }

    println!("{}", num);

    Ok(())
}
