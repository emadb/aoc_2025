use std::collections::{HashSet, VecDeque};



struct Machine {
    lights: u32,
    target_lights: u32,
    buttons: Vec<u32>,
    buttons_vec: Vec<Vec<u32>>,
    target_joltage: Vec<u32>,
    joltage: Vec<u32>
}
impl Machine {

    fn apply(&mut self, lights: u32,  btn: u32) {
        self.lights = lights ^ btn as u32;
    }

    fn apply_j(&mut self, joltage: &Vec<u32>,  btns: Vec<u32>) {

        let mut next_joltage: Vec<u32> = joltage.clone();
        for b in btns {
            next_joltage[b as usize] = joltage[b as usize] + 1;
        }

        self.joltage = next_joltage;
    }
}


fn parse_lights(l: &str) -> u32 {

    let chars = l.strip_prefix("[").unwrap().trim().strip_suffix("]").unwrap();
    chars.chars().rev().fold(0, |acc, c| {
        if c == '.' { acc << 1 } else { (acc << 1) | 1 }
    })
}

fn parse_buttons(l: &[&str]) -> Vec<u32> {
    l.iter().map(|b| {
        let chars = b.strip_prefix("(").unwrap().strip_suffix(")").unwrap();
        let parts: Vec<&str> = chars.split(",").collect();
        parts.iter()
            .map(|num | num.parse::<u64>().unwrap())
            .fold(0, |acc, num| { acc | (1 << num) })

    })
    .collect()
}

fn parse_buttons_vec(l: &[&str]) -> Vec<Vec<u32>> {
    l.iter().map(|b| {
        let chars = b.strip_prefix("(").unwrap().strip_suffix(")").unwrap();
        let parts: Vec<&str> = chars.split(",").collect();
        let bts: Vec<u32> = parts.iter().map(|num | num.parse::<u32>().unwrap()).collect();
        bts
    })
    .collect()
}

fn parse_joltage(j: &str) -> Vec<u32> {
    j.strip_prefix("{").unwrap().strip_suffix("}").unwrap().split(",").map(|jj| {
        jj.parse::<u32>().unwrap()
    }).collect()
}

fn parse_line(line: &str) -> Machine {
    let parts: Vec<&str> = line.split(" ").collect();

    let lights = parse_lights(parts[0]);
    let buttons = parse_buttons(&parts[1..parts.len() - 1]);
    let buttons_vec = parse_buttons_vec(&parts[1..parts.len() - 1]);
    let joltage = parse_joltage(parts[parts.len() - 1]);

    let len = joltage.len();

    Machine {
        lights:0,
        target_lights: lights,
        buttons,
        buttons_vec,
        target_joltage: joltage, joltage: vec![0; len] }
}

fn parse(input: String) -> Vec<Machine> {
    input.lines().map(parse_line).collect()
}


fn find_lights(machine: &mut Machine) -> u32 {
    let mut visited: HashSet<u32> = HashSet::new();
    let mut lights_queue: VecDeque<(u32, u32)> = VecDeque::new();
    lights_queue.push_back((machine.lights, 0));

    while !lights_queue.is_empty() {
        let (current_light, count) = lights_queue.pop_front().unwrap();
        if current_light == machine.target_lights {
            return count;
        }

        for btn in &machine.buttons.clone() {
            machine.apply(current_light, *btn);

            if !visited.contains(&machine.lights.clone()) {
                lights_queue.push_back((machine.lights, count + 1));
                visited.insert(machine.lights.clone());
            }
        }
    }
    0

}

pub fn part_1(input: String) -> i64 {
    let machines = parse(input);

    let mut sum = 0;
    for mut m in machines {
        let r = find_lights(&mut m);
        sum += r;
    }
    sum as i64
}

// ###################
fn gaussian_elimination(matrix: &mut Vec<Vec<f64>>) {
    let rows = matrix.len();
    if rows == 0 { return; }
    let cols = matrix[0].len();

    let mut pivot_row = 0;

    for col in 0..cols {
        if pivot_row >= rows { break; }

        // 1. Pivot Selection
        let mut max_row = pivot_row;
        for i in (pivot_row + 1)..rows {
            if matrix[i][col].abs() > matrix[max_row][col].abs() {
                max_row = i;
            }
        }

        if matrix[max_row][col].abs() < 1e-9 { continue; }

        matrix.swap(pivot_row, max_row);

        // 2. Normalize
        let pivot_val = matrix[pivot_row][col];
        for j in col..cols {
            matrix[pivot_row][j] /= pivot_val;
        }

        // 3. Eliminate
        for i in 0..rows {
            if i != pivot_row {
                let factor = matrix[i][col];
                for j in col..cols {
                    matrix[i][j] -= factor * matrix[pivot_row][j];
                }
            }
        }
        pivot_row += 1;
    }
}

fn convert_to_matrix(machine: &Machine) -> Vec<Vec<f64>> {
    let num_rows = machine.target_joltage.len();
    let num_buttons = machine.buttons_vec.len();

    let mut matrix = vec![vec![0.0; num_buttons + 1]; num_rows];

    for (col_idx, affected_counters) in machine.buttons_vec.iter().enumerate() {
        for &row_idx in affected_counters {
            if (row_idx as usize) < num_rows {
                matrix[row_idx as usize][col_idx] = 1.0;
            }
        }
    }

    for (row_idx, &target) in machine.target_joltage.iter().enumerate() {
        matrix[row_idx][num_buttons] = target as f64;
    }

    matrix
}

fn solve_recursive(
    current_free_idx: usize,
    free_cols: &Vec<usize>,
    pivot_map: &Vec<(usize, usize)>,
    current_solution: &mut Vec<f64>,
    matrix: &Vec<Vec<f64>>,
    min_presses: &mut Option<i64>
) {
    if current_free_idx == free_cols.len() {
        let mut valid = true;

        for &(row, col) in pivot_map {
            let target = matrix[row].last().unwrap();
            let mut val = *target;

            for &fc in free_cols {
                val -= matrix[row][fc] * current_solution[fc];
            }

            if val < -1e-5 || (val.round() - val).abs() > 1e-5 {
                valid = false;
                break;
            }
            current_solution[col] = val.round();
        }

        if valid {
            let sum: i64 = current_solution.iter().map(|&x| x as i64).sum();
            match min_presses {
                None => *min_presses = Some(sum),
                Some(min) => if sum < *min { *min_presses = Some(sum); }
            }
        }
        return;
    }

    let col_idx = free_cols[current_free_idx];

    for val in 0..=100 {
        current_solution[col_idx] = val as f64;
        solve_recursive(
            current_free_idx + 1,
            free_cols,
            pivot_map,
            current_solution,
            matrix,
            min_presses
        );
    }
}

fn solve_system(mut matrix: Vec<Vec<f64>>) -> Option<i64> {
    gaussian_elimination(&mut matrix);

    let rows = matrix.len();
    let cols = matrix[0].len();
    let num_vars = cols - 1;

    let mut pivot_cols = Vec::new();
    let mut pivot_map = Vec::new();

    for r in 0..rows {
        if let Some(c) = matrix[r][0..num_vars].iter().position(|&x| (x - 1.0).abs() < 1e-5) {
            pivot_cols.push(c);
            pivot_map.push((r, c));
        }
    }

    let mut free_cols = Vec::new();
    for c in 0..num_vars {
        if !pivot_cols.contains(&c) {
            free_cols.push(c);
        }
    }

    let mut min_presses = None;
    let mut current_solution = vec![0.0; num_vars];

    solve_recursive(
        0,
        &free_cols,
        &pivot_map,
        &mut current_solution,
        &matrix,
        &mut min_presses
    );

    min_presses
}

// ###################

pub fn part_2(input: String) -> i64 {
    let machines = parse(input);

    let mut sum = 0;
    for m in machines {
        let matrix = convert_to_matrix(&m);

        if let Some(presses) = solve_system(matrix) {
            sum += presses;
        }
    }
    sum as i64
}

