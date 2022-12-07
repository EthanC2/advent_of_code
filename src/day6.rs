use std::fs::File;
use std::io::{BufReader, Read};
use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
pub fn part1() {
    let file = File::open("input/day6.txt").unwrap();
    let file = BufReader::new(file);
    let mut chars = HashSet::new();
    let mut window = VecDeque::new();
    

    for (index, byte) in file.bytes().enumerate() {
        let byte = byte.unwrap();

        if chars.contains(&byte) {
            window.clear();
            chars.clear();
        } 

        if let Some(front) = window.pop_front() {
            chars.remove(front);
        }

        window.push_back(&byte);
        chars.insert(byte);
        

        println!(" - len={}", chars.len());
        if chars.len() == 4 {
            println!("Start of message found at index {}", index-2);
            break;
        }
    }
}