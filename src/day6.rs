use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn part1() -> Option<usize> {
    let mut file = File::open("input/day6.txt").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let mut chars = HashSet::new();
    for (index, window) in buffer.windows(4).enumerate() {
        window.iter().for_each(|ch| { chars.insert(*ch); });

        if chars.len() == 4 {
            return Some(index + 4);
        }

        chars.clear();
    }

    None
}

#[allow(dead_code)]
pub fn part2() -> Option<usize> {
    let mut file = File::open("input/day6.txt").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let mut chars = HashSet::new();
    for (index, window) in buffer.windows(14).enumerate() {
        window.iter().for_each(|ch| { chars.insert(*ch); });

        if chars.len() == 14 {
            return Some(index + 14);
        }

        chars.clear();
    }

    None
}