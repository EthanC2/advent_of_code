use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

//Defines an assignment [start, end] (both inclusive!)
#[derive(Debug)]
pub struct SectionAssignment {
    pub start: u8,
    pub end: u8,
}

impl FromStr for SectionAssignment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').unwrap();
    
        let start = u8::from_str(start)?;
        let end = u8::from_str(end)?;

        Ok(Self { start: start, end: end })
    }
}

#[derive(Debug)]
pub struct GroupAssignment {
    first: SectionAssignment,
    second: SectionAssignment,
}

impl FromStr for GroupAssignment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once(',').unwrap();

        let first = SectionAssignment::from_str(first)?;
        let second = SectionAssignment::from_str(second)?;

        Ok(Self { first: first, second: second })
    }
}

#[inline(always)]
#[allow(dead_code)]
fn contains_subassignment(assignment: &GroupAssignment) -> bool {
    assignment.first.start <= assignment.second.start && assignment.first.end >= assignment.second.end
    || assignment.second.start <= assignment.first.start && assignment.second.end >= assignment.first.end
}

#[allow(dead_code)]
pub fn part1() -> i32 {
    let file = File::open("input/day4.txt").unwrap();
    let file = BufReader::new(file);
    let mut overlapping_jobs = 0;

    for line in file.lines() {
        let line = line.unwrap();

        let assignment = GroupAssignment::from_str(&line).unwrap();

        if contains_subassignment(&assignment) {
            overlapping_jobs += 1;
        }
    }

    overlapping_jobs
}

#[inline(always)]
#[allow(dead_code)]
fn contains_overlap(assignment: &GroupAssignment) -> bool {
       overlaps_with_assignment(&assignment.first.start, &assignment.second)
    || overlaps_with_assignment(&assignment.first.end, &assignment.second)
    || overlaps_with_assignment(&assignment.second.start, &assignment.first) 
    || overlaps_with_assignment(&assignment.second.end, &assignment.first) 
}

#[inline(always)]
#[allow(dead_code)]
fn overlaps_with_assignment(idx: &u8, assignment: &SectionAssignment) -> bool {
	*idx >= assignment.start && *idx <= assignment.end
}

#[allow(dead_code)]
pub fn part2() -> i32 {
    let file = File::open("input/day4.txt").unwrap();
    let file = BufReader::new(file);
    let mut overlapping_jobs = 0;

    for line in file.lines() {
        let line = line.unwrap();

        let assignment = GroupAssignment::from_str(&line).unwrap();

        if contains_overlap(&assignment) {
            overlapping_jobs += 1;
        }
    }

    overlapping_jobs
}