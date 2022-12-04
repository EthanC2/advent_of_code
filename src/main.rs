#![feature(iter_array_chunks)]

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let score = day4::part2();
    println!("Total score: {score}");
}
