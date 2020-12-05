use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use itertools::Itertools;

fn main() -> std::io::Result<()> {
    let file = File::open("inputs/input1")?;
    let buf_reader = BufReader::new(file);
    let lines: Vec<u32> = buf_reader.lines().map(|s| s.unwrap().parse().unwrap()).collect();

    if let Some(v) = lines.iter().cloned().cartesian_product(lines.iter().cloned())
        .cartesian_product(lines.iter().cloned()).find(|((p1, p2), p3)| (*p1 + *p2 + *p3 == 2020)) {
        println!("{:?}", v);
    }

    Ok(())
}
