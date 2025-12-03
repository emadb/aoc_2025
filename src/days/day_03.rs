use std::arch::aarch64::uint64x2x3_t;

fn parse(input: String) -> Vec<Vec<u32>> {
    let moves: Vec<Vec<u32>> = input
        .lines()
        .map(|n| n.chars().map(|d| d.to_digit(10).unwrap()).collect())
        .collect();

    moves
}

fn find_max(nums: &Vec<u32>, start: usize, end: usize) -> (u32, usize) {
    // println!(">> {} {} {:?}", start, end, nums);
    let mut max = nums[0];
    let mut idx = 0;
    for i in (start + 1)..end {
        // println!("@ {} {}", nums[i], max);
        if nums[i] > max {
            max = nums[i];
            idx = i;
        }
    }
    (max, idx)
}

pub fn part_1(input: String) -> u32 {
    let nums = parse(input);
    let mut sum = 0;
    for n in nums {
        let (max1, idx) = find_max(&n, 0, n.len() - 1);
        // println!("max={} idx={}", max1, idx);
        let new_vec = &n[(idx + 1)..].to_vec();

        let (max2, _) = find_max(new_vec, 0, new_vec.len());
        println!("{} {} ==> {}", max1, max2, (max1 * 10) + max2);
        //
        sum = sum + (max1 * 10) + max2;
    }

    sum
}

pub fn part_2(input: String) -> u64 {
    let nums = parse(input);
    let mut sum: u64 = 0;
    for n in nums {
        let (m1, idx) = find_max(&n, 0, n.len() - 11);
        let new_vec = &n[(idx + 1)..].to_vec();

        let (m2, idx) = find_max(new_vec, 0, new_vec.len() - 10);
        let new_vec = &new_vec[(idx + 1)..].to_vec();

        let (m3, idx) = find_max(new_vec, 0, new_vec.len() - 9);
        let new_vec = &new_vec[(idx + 1)..].to_vec();

        let (m4, idx) = find_max(new_vec, 0, new_vec.len() - 8);
        let new_vec = &new_vec[(idx + 1)..].to_vec();

        let (m5, idx) = find_max(new_vec, 0, new_vec.len() - 7);
        let new_vec = &new_vec[(idx + 1)..].to_vec();

        let (m6, idx) = find_max(new_vec, 0, new_vec.len() - 6);
        let new_vec = &new_vec[(idx + 1)..].to_vec();

        let (m7, idx) = find_max(new_vec, 0, new_vec.len() - 5);
        let new_vec = &new_vec[(idx + 1)..].to_vec();

        let (m8, idx) = find_max(new_vec, 0, new_vec.len() - 4);
        let new_vec = &new_vec[(idx + 1)..].to_vec();

        let (m9, idx) = find_max(new_vec, 0, new_vec.len() - 3);
        let new_vec = &new_vec[(idx + 1)..].to_vec();

        let (m10, idx) = find_max(new_vec, 0, new_vec.len() - 2);
        let new_vec = &new_vec[(idx + 1)..].to_vec();

        let (m11, idx) = find_max(new_vec, 0, new_vec.len() - 1);
        let new_vec = &new_vec[(idx + 1)..].to_vec();

        let (m12, _) = find_max(new_vec, 0, new_vec.len());

        let str = format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}",
            m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12
        );

        sum = sum + str.parse::<u64>().unwrap();
    }

    sum
}
