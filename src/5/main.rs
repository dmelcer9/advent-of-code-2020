use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use itertools::Itertools;
use regex::Regex;

fn main() -> std::io::Result<()> {
    let file = File::open("inputs/input5")?;
    let buf_reader = BufReader::new(file);
    let lines: Vec<String> = buf_reader.lines().map(|s| s.unwrap()).collect();


    let mut vals = vec![];
    for l in lines {
        let v = l.chars().map(|c| if c == 'B' || c == 'R' {1u32} else {0} ).enumerate().map(|(idx, val)| 2u32.pow(9-(idx as u32)) * val).sum();
        vals.push(v);
    }

    vals.sort();

    let m : u32 = *vals.iter().min().unwrap();
    let mut ours = m;
    loop {
        ours += 1;
        if !vals.contains(&ours) {
            break;
        }
    }


    println!("{}", ours);
    Ok(())
}
