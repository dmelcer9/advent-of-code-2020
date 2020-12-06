use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use itertools::Itertools;
use regex::Regex;

fn good(passport: &HashMap<String, String>, key: &str, regex: &Regex) -> bool {
    if let Some(val) = passport.get(key) {
        regex.is_match(val)
    } else {
        false
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("inputs/input6")?;
    let buf_reader = BufReader::new(file);
    let lines: Vec<String> = buf_reader.lines().map(|s| s.unwrap()).collect();

    let mut all = Vec::new();
    let mut in_prog: Option<HashSet<char>> = None;

    for l in lines {
        if l.trim().is_empty() {
            if let Some(i) = in_prog.take() {
                all.push(i);
            }
        } else {
            let mut this_line = HashSet::new();
            for char in l.chars() {
                this_line.insert(char);
            }

            if let Some(i) = &mut in_prog {
                i.retain(|f| this_line.contains(f));
            } else {
                in_prog = Some(this_line)
            }
        }
    }

    if let Some(i) = in_prog {
        all.push(i);
    }


    println!("{}", all.iter().map(|n| n.len()).sum::<usize>());


    Ok(())
}
