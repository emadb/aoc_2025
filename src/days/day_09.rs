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

fn intersect((ax1, ay1): (i64, i64), (ax2, ay2): (i64, i64), (bx1, by1): (i64, i64), (bx2, by2): (i64, i64)) -> bool {
    let min_x = ax1.min(ax2);
    let max_x = ax1.max(ax2);

    let min_y = ay1.min(ay2);
    let max_y = ay1.max(ay2);

    let mix_px = bx1.min(bx2);
    let max_px = bx1.max(bx2);

    let min_py = by1.min(by2);
    let max_py = by1.max(by2);

    max_px > min_x && mix_px < max_x && max_py > min_y && min_py < max_y
}


fn is_valid_rect(x1: i64, y1: i64, x2: i64, y2: i64, lines: &Vec<Line>) -> bool {
    let mut overlap = false;
    for l in lines {
        overlap = intersect((x1, y1), (x2, y2), l.from, l.to);
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
            let width = (x1 - x2).abs() + 1;
            let height = (y1 - y2).abs() + 1;
            let this_area = width * height;

            if is_valid_rect(x1, y1, x2, y2, &lines) && this_area > area {
                area = this_area;
            }
        }
    }

    area
}
