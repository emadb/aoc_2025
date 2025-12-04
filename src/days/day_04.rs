use std::collections::HashMap;

type Grid = HashMap<(i32, i32), char>;

fn parse(input: String) -> Grid {
    let lines: Vec<&str> = input.lines().collect();

    let mut grid_map: Grid = Grid::new();

    for y in 0..lines.len() {
        let chars: Vec<char> = lines[y].chars().collect();

        for x in 0..chars.len() {
            grid_map.insert((x as i32, y as i32), chars[x]);
        }
    }
    grid_map
}

fn count_neighbors(grid: &Grid, x: i32, y: i32) -> u8 {
    let neighbors: Vec<(i8, i8)> = vec![
        (-1, 0),
        (1, 0),
        (-1, -1),
        (1, 1),
        (0, -1),
        (0, 1),
        (-1, 1),
        (1, -1),
    ];
    let mut counter = 0;
    for (dx, dy) in neighbors {
        let found = match &grid.get(&(x + dx as i32, y + dy as i32)) {
            Some('@') => 1,
            Some(_) => 0,
            None => 0,
        };
        counter += found
    }
    counter
}

pub fn part_1(input: String) -> u64 {
    let grid = parse(input);
    let mut count = 0;
    for cell in &grid {
        if *cell.1 == '@' {
            let (x, y) = cell.0;
            if count_neighbors(&grid, *x, *y) < 4 {
                count += 1;
            }
        }
    }

    count
}

pub fn part_2(input: String) -> u64 {
    let mut grid = parse(input);
    let mut count = 0;
    let mut founded: Vec<(i32, i32)> = vec![];

    loop {
        for cell in &grid {
            if *cell.1 == '@' {
                let (x, y) = cell.0;
                if count_neighbors(&grid, *x, *y) < 4 {
                    count += 1;
                    founded.push((*x, *y));
                }
            }
        }
        for (xx, yy) in &founded {
            let _ = &grid.insert((*xx, *yy), '.');
        }
        if founded.len() == 0 {
            break;
        }
        founded.clear();
    }

    count
}
