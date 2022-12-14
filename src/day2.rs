use std::str::FromStr;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::result::Result;
use std::error::Error;

#[derive(PartialEq, Eq)]
enum RoundOutcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Debug)]
pub struct ParseRoundOutcomeError;

impl FromStr for RoundOutcome {
    type Err = ParseRoundOutcomeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RoundOutcome::Loss),
            "Y" => Ok(RoundOutcome::Draw),
            "Z" => Ok(RoundOutcome::Win),
            _ => Err(ParseRoundOutcomeError)
        }
    }
}

#[derive(PartialEq, Eq)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug)]
pub struct ParseChoiceError;

impl FromStr for Choice {
    type Err = ParseChoiceError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Choice::Rock),
            "B" | "Y" => Ok(Choice::Paper),
            "C" | "Z" => Ok(Choice::Scissors),
            _ => Err(ParseChoiceError)
        }
    }
}

#[allow(dead_code)]
fn play_round(opponent: &Choice, you: &Choice) -> RoundOutcome {
    match (opponent, you) {
        (Choice::Rock, Choice::Paper) => RoundOutcome::Win,
        (Choice::Rock, Choice::Scissors) => RoundOutcome::Loss,
        (Choice::Rock, Choice::Rock) => RoundOutcome::Draw,
        (Choice::Paper, Choice::Scissors) => RoundOutcome::Win,
        (Choice::Paper, Choice::Rock) => RoundOutcome::Loss,
        (Choice::Paper, Choice::Paper) => RoundOutcome::Draw,
        (Choice::Scissors, Choice::Rock) => RoundOutcome::Win,
        (Choice::Scissors, Choice::Paper) => RoundOutcome::Loss,
        (Choice::Scissors, Choice::Scissors) => RoundOutcome::Draw,
    }
}

#[allow(dead_code)]
pub fn part1() -> Result<u32, Box<dyn Error>> {
    let file = File::open("input/day2.txt")?;
    let buffer = BufReader::new(file);
    let mut score = 0;

    for line in buffer.lines() {
        let line = line.unwrap();
        let (opp, you) = line.split_once(' ').unwrap();
        let opp = opp.parse::<Choice>().unwrap();
        let you = you.parse::<Choice>().unwrap();
        let result = play_round(&opp, &you);
        score += (you as u32) + (result as u32);
    }

    Ok(score)
}

#[allow(dead_code)]
fn imply(op_choice: &Choice, outcome: &RoundOutcome) -> Choice {
    match (op_choice, outcome) {
        (Choice::Rock, RoundOutcome::Win) => Choice::Paper,
        (Choice::Rock, RoundOutcome::Loss) => Choice::Scissors,
        (Choice::Rock, RoundOutcome::Draw) => Choice::Rock,
        (Choice::Paper, RoundOutcome::Win) => Choice::Scissors,
        (Choice::Paper, RoundOutcome::Loss) => Choice::Rock,
        (Choice::Paper, RoundOutcome::Draw) => Choice::Paper,
        (Choice::Scissors, RoundOutcome::Win) => Choice::Rock,
        (Choice::Scissors, RoundOutcome::Loss) => Choice::Paper,
        (Choice::Scissors, RoundOutcome::Draw) => Choice::Scissors,
    }
}

#[allow(dead_code)]
pub fn part2() -> Result<u32, Box<dyn Error>> {
    let file = File::open("input/day2.txt")?;
    let buffer = BufReader::new(file);
    let mut score = 0;

    for line in buffer.lines() {
        let line = line.unwrap();
        let (opp, outcome) = line.split_once(' ').unwrap();
        let opp = opp.parse::<Choice>().unwrap();
        let outcome = outcome.parse::<RoundOutcome>().unwrap();
        let result = imply(&opp, &outcome);
        score += (outcome as u32) + (result as u32);
    }

    Ok(score)
}