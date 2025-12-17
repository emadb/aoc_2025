#[derive(Debug)]
struct Region {
    width: i32,
    length: i32,
    shape_qty: Vec<i32>
}

impl From<&str> for Region {
    fn from(value: &str) -> Self {
        let (size, shapes) = value.split_once(": ").unwrap();
        let (w, h) = size.split_once("x").unwrap();
        let qtys = shapes.split(" ").map(|q| { q.parse::<i32>().unwrap()}).collect();
        Self { width: w.parse::<i32>().unwrap(), length: h.parse().unwrap(), shape_qty: qtys }
    }
}

fn parse(input: String) -> Vec<Region> {
     let lines: Vec<&str> = input.split("\n\n").collect();
     let regions = lines[lines.len() - 1];

     regions.split("\n").map(|r| {
         Region::from(r)
     }).collect()
}

fn is_big_enough(r: &Region) -> bool {
    (r.width * r.length) >= (r.shape_qty.iter().sum::<i32>() * 9)
}

pub fn part_1(input: String) -> i64 {
    let regions = parse(input);

   regions.iter().filter(|r| {
        is_big_enough(r)
    }).count() as i64
}
