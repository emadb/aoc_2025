enum Direction {
    L,
    R,
}

struct Move {
    direction: Direction,
    steps: i32,
}

fn parse(input: String) -> Vec<Move> {
    let moves: Vec<Move> = input
        .lines()
        .map(|l| {
            let (d, s) = l.split_at(1);
            match d {
                "L" => Move {
                    direction: Direction::L,
                    steps: s.parse().unwrap(),
                },
                "R" => Move {
                    direction: Direction::R,
                    steps: s.parse().unwrap(),
                },
                _ => panic!(),
            }
        })
        .collect();

    moves
}

pub fn part_1(input: String) -> i32 {
    let moves = parse(input);

    let mut position = 50;
    let mut zeros = 0;
    for m in moves {
        position = match m.direction {
            Direction::L => (position - m.steps) % 100,
            Direction::R => (position + m.steps) % 100,
        };
        if position == 0 {
            zeros = zeros + 1
        }
    }

    zeros
}

pub fn part_2(input: String) -> i32 {
    let moves = parse(input);

    let mut position = 50;
    let mut zeros = 0;
    for m in moves {
        for _ in 0..m.steps {
            position = match m.direction {
                Direction::L => (position - 1) % 100,
                Direction::R => (position + 1) % 100,
            };
            if position == 0 {
                zeros = zeros + 1;
            }
        }
    }

    zeros
}
