#![feature(iter_array_chunks)]
#![feature(iter_advance_by)]
#![feature(let_chains)]

mod day7;

fn main() {
    let size = day7::part1();
    println!("Total filesystem size: {size}");
}
