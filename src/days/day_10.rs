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
    // fn press_index(&mut self, btn_index: usize) {
    //     self.lights = self.lights ^ self.buttons[btn_index] as u32;

    // }
    // fn press(&mut self, btn: u8) {
    //     self.lights = self.lights ^ btn as u32;
    // }

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
fn are_equal(one: &Vec<u32>, two: &Vec<u32>) -> bool {
    if one.len() != two.len() {
        return false;
    }
    for i in 0..one.len() {
        if one[i] != two[i] {
            return false;
        }
    }
    return true;
}

fn find_joltages(machine: &mut Machine) -> u32 {
    let mut visited: HashSet<Vec<u32>> = HashSet::new();
    let mut joltage_queue: VecDeque<(Vec<u32>, u32)> = VecDeque::new();
    joltage_queue.push_back((machine.joltage.clone(), 0));

    while !joltage_queue.is_empty() {
        let (current_joltage, count) = joltage_queue.pop_front().unwrap();
        if are_equal(&current_joltage, &machine.target_joltage) {
            return count;
        }

        for btn in &machine.buttons_vec.clone() {
            machine.apply_j(&current_joltage, btn.clone());
            if !visited.contains(&machine.joltage.clone()) && is_valid(machine) {
                joltage_queue.push_back((machine.joltage.clone(), count + 1));
                visited.insert(machine.joltage.clone());
            }
        }
    }
    0

}

fn is_valid(machine: &mut Machine) -> bool {
    for i in 0..machine.joltage.len() {
        if machine.joltage[i] > machine.target_joltage[i] {
            return false
        }
    }
    return true;
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

pub fn part_2(input: String) -> i64 {
    let machines = parse(input);

    let mut sum = 0;
    for mut m in machines {
        let r = find_joltages(&mut m);
        println!("> {}", r);
        sum += r;
    }
    sum as i64
}

