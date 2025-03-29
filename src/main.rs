mod days;

use days::day04::*;

use std::fs;

// const INPUTS_PATH :&str = "/Users/scryptan/programming/rust/advent21/src/inputs/";
const INPUTS_PATH :&str = "D:\\\\AllStuff/Experiments/Rust/aoc_rust/src/inputs/";

fn main() {
    let str = fs::read_to_string(INPUTS_PATH.to_owned() + "day4.txt")
    .expect("Can't find file");

    let input = parse(str.as_str());
    println!("First part {}", part1(&input));
    println!("Second part {}", part2(&input));
}