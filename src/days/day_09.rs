use std::cmp;

fn parse(input: String) -> Vec<(i64, i64)> {
    let coords = input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    coords
}

pub fn part_1(input: String) -> i64 {
    let coords = parse(input);

    let mut area = 0;
    for i in 0..coords.len() {
        for j in 1..coords.len() {
            let (x1, y1) = coords[i];
            let (x2, y2) = coords[j];
            let this_area = (x2 - x1 + 1).abs() * (y2 - y1 + 1).abs();
            if this_area > area {
                area = this_area
            }
        }
    }

    area
}

fn ccw(a: (i64, i64), b: (i64, i64), c: (i64, i64)) -> bool {
    (c.1 - a.1) * (b.0 - a.0) > (b.1 - a.1) * (c.0 - a.0)
}

fn is_valid_rect(x1: i64, y1: i64, x2: i64, y2: i64, lines: &Vec<Line>) -> bool {
    let mut overlap = false;
    for l in lines {
        overlap = ccw((x1, y1), (l.from.0, l.from.1), (l.to.0, l.to.1)) != ccw((x2, y2), (l.from.0, l.from.1), (l.to.0, l.to.1))
            && ccw((x1, y1), (x2, y2), (l.from.0, l.from.1)) != ccw((x1, y1), (x2, y2), (l.to.0, l.to.1));

        // println!("x1=({}, {}) x2=({}, {}) from=({},{}) to=({},{}) => {}", x1, y1, x2, y2, l.from.0, l.from.1, l.to.0, l.to.1, overlap);

        if overlap {
            break;
        }
    }

    !overlap
}

#[derive(Debug, Clone)]
struct Line {
    from: (i64, i64),
    to: (i64, i64),
}

fn build_vertex(coords: &Vec<(i64, i64)>) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();
    for i in 0..coords.len() - 1 {
        let line = Line {
            from: coords[i],
            to: coords[i + 1],
        };
        lines.push(line)
    }
    lines.push(Line {
         from: *coords.last().unwrap(),
         to: coords[0],
     });
    lines
}

pub fn part_2(input: String) -> i64 {
    let coords = parse(input);

    let lines = build_vertex(&coords);

    let mut area = 0;
    for i in 0..coords.len() {
        for j in 1..coords.len() {
            let (x1, y1) = coords[i];
            let (x2, y2) = coords[j];
            let this_area = (x2 - x1 + 1).abs() * (y2 - y1 + 1).abs();
            if is_valid_rect(x1, y1, x2, y2, &lines) && this_area > area {
                println!("x1={} y1={} x2={} y2={} area={}", x1, y1, x2, y2, this_area);
                area = this_area;
            }
        }
    }

    area
}
