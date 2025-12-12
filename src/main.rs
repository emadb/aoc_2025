pub mod days;
use crate::days::day_10::part_2;
use aoc_2025::read_file;

fn main() {
    let content = read_file(10);
    let res = part_2(content);
    println!("> {}", res);
}
