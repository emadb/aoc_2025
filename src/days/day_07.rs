use std::collections::HashMap;
use std::{
    collections::{HashSet, VecDeque},
};

fn parse(input: String) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub fn part_1(input: String) -> u64 {
    let grid = parse(input);

    let start_x = grid[0].iter().position(|c| *c == 'S').unwrap();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut splits = 0;
    let mut queue = VecDeque::from([(start_x, 0)]);
    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();

        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));

        if grid[y][x] == '^' {
            splits += 1;
            if x + 1 < grid[y].len() {
                queue.push_back((x + 1, y));
            }
            if x - 1 > 0 {
                queue.push_back((x - 1, y));
            }
        } else {
            if y + 1 < grid.len() {
                queue.push_back((x, y + 1));
            }
        }
    }

    splits
}


fn search_paths(x: usize, y: usize, grid: &[Vec<char>], memoized: &mut HashMap<(usize, usize), u64>) -> u64 {
    if y >= grid.len() {
        return 1;
    }

    if let Some(&v) = memoized.get(&(x, y)) {
        return v;
    }

    let mut paths = 0;
    if grid[y][x] == '^' {
        if x + 1 < grid[y].len() {
            paths += search_paths(x + 1, y, grid, memoized);
        }
        if x > 0 {
            paths += search_paths(x - 1, y, grid, memoized);
        }
    } else {
        if y + 1 <= grid.len() {
            paths += search_paths(x, y + 1, grid, memoized);
        }
    }

    memoized.insert((x, y), paths);
    paths
}

pub fn part_2(input: String) -> u64 {
    let grid = parse(input);

    let start_x = grid[0].iter().position(|c| *c == 'S').unwrap();
    let mut memoized: HashMap<(usize, usize), u64> = HashMap::new();

    search_paths(start_x, 0, &grid, &mut memoized)
}
