use std::collections::BinaryHeap;
use std::io::{BufRead, BufReader, Result};
use std::fs::File;

#[allow(dead_code)]
pub fn part1() -> Result<u32> {
    let file = File::open("input/day1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut max_calories = u32::MIN;
    let mut curr_calories = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            max_calories = max_calories.max(curr_calories);
            curr_calories = 0;
        } else {
            curr_calories += line.parse::<u32>().unwrap();
        }
    }

    return Ok(max_calories);
}

#[allow(dead_code)]
pub fn part2() -> Result<u32> {
    let file = File::open("input/day1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut heap = BinaryHeap::new();

    
    let mut calories = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            heap.push(calories);
            calories = 0;
        } else {
            calories += line.parse::<u32>().unwrap();
        }
    }

    let mut sum = 0;
    for _ in 1..=3 {
        sum += heap.pop().unwrap();
    }

    return Ok(sum);
}