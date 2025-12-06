fn parse(input: String) -> Vec<Vec<String>> {
    let mut lines: Vec<Vec<String>> = vec![];
    for l in input.lines() {
        let parts = l.split(" ");

        let mut line: Vec<String> = Vec::new();
        for p in parts.filter(|p| !p.is_empty()) {
            line.push(String::from(p));
        }
        lines.push(line);
    }
    lines.reverse();
    lines
}

pub fn part_1(input: String) -> u64 {
    let lines = parse(input);
    let width = lines[0].len();

    let mut total = 0;

    for col_index in 0..width {
        let op = &lines[0][col_index];
        let mut result = 0;
        for row_index in 1..lines.len() {
            if col_index < lines[row_index].len() {
                let value = &lines[row_index][col_index];
                if op == "+" {
                    result = result + value.parse::<u64>().unwrap();
                } else {
                    if result == 0 {
                        result = 1
                    };
                    result = result * value.parse::<u64>().unwrap();
                }
            }
        }
        total += result;
    }

    total
}

fn parse_2(lines: Vec<&str>) -> Vec<Vec<char>> {
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let mut chars: Vec<char> = line.chars().collect();
        while chars.len() < width {
            chars.push(' ');
        }
        grid.push(chars);
    }

    grid.reverse();
    grid
}

pub fn part_2(input: String) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let grid = parse_2(lines.clone());
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let mut total = 0;

    let mut current_nums: Vec<u64> = Vec::new();
    let mut current_op: char = '0';

    for col in (0..width).rev() {
        let is_separator = grid.iter().all(|row| row[col] == ' ');

        if is_separator {
            if !current_nums.is_empty() {
                total += apply_op(&current_nums, current_op);
                current_nums.clear();
                current_op = '0';
            }
        } else {
            let op_char = grid[0][col];
            if op_char == '+' || op_char == '*' {
                current_op = op_char;
            }

            let mut num_str = String::new();
            for row_idx in (1..grid.len()).rev() {
                let c = grid[row_idx][col];
                if c.is_ascii_digit() {
                    num_str.push(c);
                }
            }

            if !num_str.is_empty() {
                let num = num_str.parse::<u64>().unwrap();
                current_nums.push(num);
            }
        }
    }

    total += apply_op(&current_nums, current_op);

    total
}

fn apply_op(nums: &[u64], op: char) -> u64 {
    match op {
        '+' => nums.iter().sum(),
        '*' => nums.iter().product(),
        _ => 0,
    }
}
