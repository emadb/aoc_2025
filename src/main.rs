pub mod days;
use crate::days::day_01::part_1;
use aoc_2025::read_file;

fn main() {
    let content = read_file(1);
    let res = part_1(content);
    println!("> {}", res);
}
