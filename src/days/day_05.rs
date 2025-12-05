use std::cmp;

fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    input
        .lines()
        .map(|l| {
            let (from, to) = l.split_once("-").unwrap();
            (from.parse().unwrap(), to.parse().unwrap())
        })
        .collect()
}

fn parse_ingredients(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn merge_overlapping_ranges(ranges: &mut Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    ranges.sort_by(|r1, r2| r1.0.cmp(&r2.0));

    let mut merged: Vec<(u64, u64)> = vec![];
    merged.push(ranges[0]);

    for ri in 1..ranges.len() {
        let current_range: (u64, u64) = ranges[ri].clone();
        let rj: usize = merged.len() - 1;

        if current_range.0 >= merged[rj].0 && current_range.0 <= merged[rj].1 {
            merged[rj].1 = cmp::max(current_range.1, merged[rj].1);
        } else {
            merged.push(current_range);
        }
    }
    merged
}

pub fn part_1(input: String) -> u64 {
    let (p1, p2) = input.split_once("\n\n").unwrap();
    let ranges = parse_ranges(p1);
    let ingredients = parse_ingredients(p2);

    let mut count = 0;
    for ing in ingredients {
        let found = ranges.iter().find(|(from, to)| from <= &ing && to >= &ing);
        let sum = match found {
            Some(_) => 1,
            None => 0,
        };
        count += sum
    }
    count
}

pub fn part_2(input: String) -> u64 {
    let (p1, _) = input.split_once("\n\n").unwrap();
    let mut ranges = parse_ranges(p1);
    let merged_ranges = merge_overlapping_ranges(&mut ranges);

    merged_ranges
        .iter()
        .fold(0, |acc, (r1, r2)| acc + (r2 - r1) + 1)
}
