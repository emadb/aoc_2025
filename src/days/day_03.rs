fn parse(input: String) -> Vec<Vec<u32>> {
    let moves: Vec<Vec<u32>> = input
        .lines()
        .map(|n| n.chars().map(|d| d.to_digit(10).unwrap()).collect())
        .collect();

    moves
}

fn find_max(nums: &Vec<u32>, end: usize) -> (u32, usize) {
    let mut max = nums[0];
    let mut idx = 0;

    for i in 1..end {
        if nums[i] > max {
            max = nums[i];
            idx = i;
        }
    }
    (max, idx)
}

fn find_n_digits(line: Vec<u32>, digits: usize) -> u64 {
    let (_, number) = (0..digits).rev().fold((line, 0 as u64), |(list, sum), i| {
        let (max, idx) = find_max(&list, list.len() - i);
        let new_vec = list[(idx + 1)..].to_vec().clone();
        let new_acc: u64 = sum + (max as u64) * (10 as u64).pow(i as u32);

        (new_vec, new_acc)
    });
    number
}

pub fn part_1(input: String) -> u64 {
    let lines = parse(input);
    lines
        .into_iter()
        .fold(0, |acc, n| acc + find_n_digits(n, 2))
}

pub fn part_2(input: String) -> u64 {
    let lines = parse(input);
    lines
        .into_iter()
        .fold(0, |acc, n| acc + find_n_digits(n, 12))
}
