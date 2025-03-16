mod days;

use days::*;

use std::fs;

const INPUTS_PATH :&str = "/Users/scryptan/programming/rust/advent21/src/inputs/";

fn main() {
    let str = fs::read_to_string(INPUTS_PATH.to_owned() + "day3.txt")
    .expect("Can't find file");
    let input = day03::parse(str.as_str());
    println!("First part {}", day03::part1(&input));
    println!("Second part {}", day03::part2(&input));
}