use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

#[allow(dead_code)]
fn calc_priority(ch: &char) -> i32 {
    if ch.is_ascii_lowercase() {
        ((*ch as i32) - ('a' as i32)) + 1
    } else {
        ((*ch as i32) - ('A' as i32)) + 27
    }
}

#[allow(dead_code)]
pub fn part1() -> i32 {
    let file = File::open("input/day3.txt").unwrap();
    let file = BufReader::new(file);

    let mut fst_hs = HashSet::new();   
    let mut scnd_hs = HashSet::new();
    let mut sum_intersection = 0;

    for line in file.lines() {
        let line = line.unwrap();

        let mid = line.len() / 2;
        let (fst_compart, scnd_compart) = line.split_at(mid);

        fst_compart.chars().for_each(|ch| { fst_hs.insert(ch); });
        scnd_compart.chars().for_each(|ch| { scnd_hs.insert(ch); });

        for ch in fst_hs.intersection(&scnd_hs) {
            sum_intersection += calc_priority(&ch);
        }

        fst_hs.clear();
        scnd_hs.clear();
    }

    sum_intersection
}

#[allow(dead_code)]
pub fn part2() -> i32 {
    let file = File::open("input/day3.txt").unwrap();
    let file = BufReader::new(file);

    let mut fst_hs = HashSet::new();   
    let mut scnd_hs = HashSet::new();
    let mut thrd_hs = HashSet::new();
    let mut sum_intersection = 0;

    for [line_one, line_two, line_three] in file.lines().array_chunks() {
        line_one.unwrap().chars().for_each(|ch| { fst_hs.insert(ch); });
        line_two.unwrap().chars().for_each(|ch| { scnd_hs.insert(ch); });
        line_three.unwrap().chars().for_each(|ch| { thrd_hs.insert(ch); });

        let int = fst_hs.intersection(&scnd_hs).map(|ch| *ch).collect::<HashSet<char>>();  
        for ch in int.intersection(&thrd_hs) {
            sum_intersection += calc_priority(&ch);
        }

        fst_hs.clear();
        scnd_hs.clear();
        thrd_hs.clear();
    }

    sum_intersection
}