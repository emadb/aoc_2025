#[derive(Debug)]
struct Range {
    from: u64,
    to: u64,
}

fn parse(input: String) -> Vec<Range> {
    let ranges: Vec<Range> = input
        .split(",")
        .map(|c| {
            let parts: Vec<&str> = c.splitn(2, '-').collect();
            let from = parts[0].parse::<u64>().unwrap();
            let to = parts[1].parse::<u64>().unwrap();
            Range { from, to }
        })
        .collect();
    ranges
}

fn contains_repetition(num: u64) -> bool {
    let str = num.to_string();
    if str.len() % 2 != 0 {
        false
    } else {
        let (p1, p2) = str.split_at(num.to_string().len() / 2);
        p1 == p2
    }
}

fn contains_multiple_repetition(num: u64) -> bool {
    let str: String = num.to_string();
    let str_len = str.len();
    for i in 1..str_len {
        let part: &str = &str.as_str()[..i];
        if str == part.repeat(str_len / i) {
            return true;
        }
    }
    false
}

pub fn part_1(input: String) -> u64 {
    let ranges = parse(input);
    let mut sum: u64 = 0;
    for r in ranges {
        for i in r.from..r.to + 1 {
            if contains_repetition(i as u64) {
                sum = sum + i as u64;
            }
        }
    }
    sum
}

pub fn part_2(input: String) -> u64 {
    let ranges = parse(input);
    let mut sum: u64 = 0;
    for r in ranges {
        for i in r.from..r.to + 1 {
            if contains_multiple_repetition(i as u64) {
                sum = sum + i as u64;
            }
        }
    }
    sum
}
