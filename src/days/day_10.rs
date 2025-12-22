use std::collections::{HashSet, VecDeque};

struct Machine {
    lights: u32,
    target_lights: u32,
    buttons: Vec<u32>,
    buttons_vec: Vec<Vec<u32>>,
    target_joltage: Vec<i32>,
    joltage: Vec<u32>,
}
impl Machine {
    fn apply(&mut self, lights: u32, btn: u32) {
        self.lights = lights ^ btn as u32;
    }

    fn apply_j(&mut self, joltage: &Vec<u32>, btns: Vec<u32>) {
        let mut next_joltage: Vec<u32> = joltage.clone();
        for b in btns {
            next_joltage[b as usize] = joltage[b as usize] + 1;
        }

        self.joltage = next_joltage;
    }
}

fn parse_lights(l: &str) -> u32 {
    let chars = l
        .strip_prefix("[")
        .unwrap()
        .trim()
        .strip_suffix("]")
        .unwrap();
    chars
        .chars()
        .rev()
        .fold(0, |acc, c| if c == '.' { acc << 1 } else { (acc << 1) | 1 })
}

fn parse_buttons(l: &[&str]) -> Vec<u32> {
    l.iter()
        .map(|b| {
            let chars = b.strip_prefix("(").unwrap().strip_suffix(")").unwrap();
            let parts: Vec<&str> = chars.split(",").collect();
            parts
                .iter()
                .map(|num| num.parse::<u64>().unwrap())
                .fold(0, |acc, num| acc | (1 << num))
        })
        .collect()
}

fn parse_buttons_vec(l: &[&str]) -> Vec<Vec<u32>> {
    l.iter()
        .map(|b| {
            let chars = b.strip_prefix("(").unwrap().strip_suffix(")").unwrap();
            let parts: Vec<&str> = chars.split(",").collect();
            let bts: Vec<u32> = parts
                .iter()
                .map(|num| num.parse::<u32>().unwrap())
                .collect();
            bts
        })
        .collect()
}

fn parse_joltage(j: &str) -> Vec<i32> {
    j.strip_prefix("{")
        .unwrap()
        .strip_suffix("}")
        .unwrap()
        .split(",")
        .map(|jj| jj.parse::<i32>().unwrap())
        .collect()
}

fn parse_line(line: &str) -> Machine {
    let parts: Vec<&str> = line.split(" ").collect();

    let lights = parse_lights(parts[0]);
    let buttons = parse_buttons(&parts[1..parts.len() - 1]);
    let buttons_vec = parse_buttons_vec(&parts[1..parts.len() - 1]);
    let joltage = parse_joltage(parts[parts.len() - 1]);

    let len = joltage.len();

    Machine {
        lights: 0,
        target_lights: lights,
        buttons,
        buttons_vec,
        target_joltage: joltage,
        joltage: vec![0; len],
    }
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
                visited.insert(machine.lights);
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

fn get_combinations(set: &[u32], count: usize) -> Vec<Vec<u32>> {
    if count == 0 {
        vec![Vec::new()]
    } else {
        set[..set.len() - count + 1]
            .iter()
            .enumerate()
            .flat_map(|(i, &t)| {
                get_combinations(&set[i + 1..], count - 1)
                    .iter()
                    .map(|c| {
                        let mut c1 = c.clone();
                        c1.push(t);
                        c1
                    })
                    .collect::<Vec<Vec<u32>>>()
            })
            .collect()
    }
}

fn subsets(set: &[u32]) -> Vec<Vec<u32>> {
    let mut subsets: Vec<Vec<u32>> = Vec::new();
    for count in 0..=set.len() {
        subsets.extend(get_combinations(set, count));
    }
    subsets
}

fn find_presses(machine: &Machine) -> usize {
    let xors: Vec<(Vec<u32>, u32)> = subsets(&machine.buttons)
        .iter()
        .map(|s| (s.to_owned(), s.iter().fold(0, |a, &b| a ^ b)))
        .collect();
    find_presses_r(&xors, &machine.target_joltage).unwrap()
}

fn find_presses_r(subset_xors: &[(Vec<u32>, u32)], joltages: &Vec<i32>) -> Option<usize> {
    if joltages.iter().all(|&j| j == 0) {
        return Some(0);
    }
    let binary_joltages = joltages
        .iter()
        .enumerate()
        .map(|(i, j)| if j % 2 != 0 { 1 << i } else { 0 })
        .sum();
    let mut best = None;
    for (subset, xor) in subset_xors {
        if *xor == binary_joltages {
            let mut mask = 1;
            let mut new_joltages = Vec::new();
            for &joltage in joltages {
                new_joltages
                    .push((joltage - subset.iter().filter(|&b| b & mask != 0).count() as i32) / 2);
                mask <<= 1;
            }
            if new_joltages.iter().all(|&j| j >= 0) {
                let press_count =
                    find_presses_r(subset_xors, &new_joltages).map(|c| subset.len() + 2 * c);
                best = best.min(press_count).or(best).or(press_count);
            }
        }
    }
    best
}

pub fn part_2(input: String) -> i64 {
    let machines = parse(input);

    let mut sum = 0;
    for m in machines {
        let r = find_presses(&m);
        sum += r;
    }
    sum as i64
}
