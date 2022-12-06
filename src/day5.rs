use std::iter;
use std::fs::File;
use std::io::{Read, BufRead, BufReader};
use std::collections::VecDeque;

#[allow(dead_code)]
fn read_crates(file: &mut BufReader<File>) -> Vec<VecDeque<char>> {
    let mut stacks = iter::repeat(VecDeque::new()).take(9).collect::<Vec<VecDeque<char>>>();

    for line in file.lines().take(8) {
        let line = line.unwrap();

        let mut iter = line.chars().enumerate();
        iter.advance_by(1).unwrap();
        for (idx, chr) in iter.step_by(4) {
            if !chr.is_ascii_whitespace() {
                stacks.get_mut((idx-1)/4).unwrap().push_front(chr);
            }
        }
    }
    
    file.lines().advance_by(2).unwrap();

    stacks
}

#[allow(dead_code)]
fn parse_instructions(text: &str) -> (usize, usize, usize) {
    let nums = text.split_ascii_whitespace()
        .filter(|s| s.chars().all(|ch| ch.is_ascii_digit()))
        .map(|num_str| num_str.parse::<usize>().expect("cannot fail"))
        .collect::<Vec<usize>>();

    (nums[0], nums[1], nums[2])
}

#[allow(dead_code)]
pub fn part1() -> String {
    let file = File::open("input/day5.txt").unwrap();
    let mut file = BufReader::new(file);
    
    let mut stacks = read_crates(file.by_ref());

    for line in file.lines() {
        let line = line.unwrap();
        let (num_crates, source, destination) = parse_instructions(&line);
        
        for _ in 0..num_crates {
            if let Some(crate_) = stacks[source-1].pop_back() {
                stacks[destination-1].push_back(crate_);
            }
        }
    }
    

    String::from_iter(stacks.iter().map(|stack| stack.back().unwrap()))
}

#[allow(dead_code)]
pub fn part2() -> String {
    let file = File::open("input/day5.txt").unwrap();
    let mut file = BufReader::new(file);

    let mut stacks = read_crates(file.by_ref());

    for line in file.lines() {
        let line = line.unwrap();
        let (num_crates, source, destination) = parse_instructions(&line);

        //TODO: avoid cloning and address double mutable borrow
        let n_last = stacks[source-1].len() - num_crates;
        let crates = stacks[source-1].drain(n_last..).collect::<Vec<char>>();
        stacks[destination-1].extend(crates);
    }

    String::from_iter(stacks.iter().map(|stack| stack.back().unwrap()))
}