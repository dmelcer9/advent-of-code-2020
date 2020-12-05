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
    let file = File::open("inputs/input4")?;
    let buf_reader = BufReader::new(file);
    let lines: Vec<String> = buf_reader.lines().map(|s| s.unwrap()).collect();

    let mut all = Vec::new();
    let mut in_prog = HashMap::new();

    for l in lines {
        if l.trim().is_empty() {
            all.push(in_prog);
            in_prog = HashMap::new();
        } else {
            for kv in l.split(" ") {
                let mut kvsplit = kv.split(":");
                let key = kvsplit.next().unwrap();
                let val = kvsplit.next().unwrap();
                in_prog.insert(key.to_owned(), val.to_owned());
            }
        }
    }

    if !in_prog.is_empty() {
        all.push(in_prog)
    }

    let byr_re = Regex::new(r"^(19[2-8][0-9]|199[0-9]|200[0-2])$").unwrap();
    let iyr_re = Regex::new(r"^(201[0-9]|2020)$").unwrap();
    let eyr_re = Regex::new(r"^(202[0-9]|2030)$").unwrap();
    let hgt_re = Regex::new(r"^((1[5-8][0-9]|19[0-3])cm)|((59|6[0-9]|7[0-6])in)$").unwrap();
    let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let ecl_re = Regex::new(r"^(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)$").unwrap();
    let pid_re = Regex::new(r"^\d{9}$").unwrap();

    let num = all.iter().filter(|passport| {
        good(&passport, "byr", &byr_re) &
            good(&passport, "iyr", &iyr_re) &
            good(&passport, "eyr", &eyr_re) &
            good(&passport, "hgt", &hgt_re) &
            good(&passport, "hcl", &hcl_re) &
            good(&passport, "ecl", &ecl_re) &
            good(&passport, "pid", &pid_re)
    }).count();


    println!("{}", num);

    Ok(())
}
