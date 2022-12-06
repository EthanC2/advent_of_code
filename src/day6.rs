use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::collections::HashSet;

#[allow(dead_code)]
pub fn part1() {
    let file = File::open("input/day6.txt").unwrap();
    let file = BufReader::new(file);
    
    if let Some(Ok(byte)) = file.bytes().last() {
        println!("{:?}", byte)
    }
}