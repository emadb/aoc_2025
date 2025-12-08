use std::collections::HashSet;

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
struct Junction {
    x: i64,
    y: i64,
    z: i64,
}


struct Dist {
    j1: Junction,
    j2: Junction,
    d: i64,
}

fn distance(a: &Junction, b: &Junction) -> i64 {
    let x2 = (a.x - b.x).pow(2);
    let y2 = (a.y - b.y).pow(2);
    let z2 = (a.z - b.z).pow(2);

    (x2 + y2 + z2).isqrt()
}

fn parse(input: String) -> Vec<Junction> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(",");
            let [x, y, z] = [(); 3].map(|()| parts.next().unwrap().parse::<i64>().unwrap());
            Junction { x, y, z }
        })
        .collect()
}

fn find_hashset_index(j: &Junction, circuits: &Vec<HashSet<Junction>>) -> Option<usize> {
    for i in 0..circuits.len() {
        if circuits[i].contains(j) {
            return Some(i);
        }
    }
    None
}

fn get_distances(junctions: &Vec<Junction>) -> Vec<Dist>{
    let mut distances: Vec<Dist> = vec![];
    for i in 0..junctions.len() {
        for j in (i + 1)..junctions.len() {
            let dist = distance(&junctions[i], &junctions[j]);
            distances.push(Dist {
                j1: junctions[i],
                j2: junctions[j],
                d: dist,
            });
        }
    }
    distances.sort_by(|a, b| a.d.cmp(&b.d));
    distances
}

fn associate_to_circuits(d: &Dist, h1:  Option<usize>, h2:  Option<usize>, circuits: &mut Vec<HashSet<Junction>>) {
    match (h1, h2) {
        (Some(i1), Some(i2)) => {
            if i1 != i2 {
                let (low, high) = if i1 < i2 { (i1, i2) } else { (i2, i1) };
                let other = circuits.remove(high);
                circuits[low].extend(other.into_iter());

            }

        }
        (Some(i1), None) => {
            circuits[i1].insert(d.j2);
        }
        (None, Some(i2)) => {
            circuits[i2].insert(d.j1);
        }
        (None, None) => {
            let mut new_h = HashSet::new();
            new_h.insert(d.j1);
            new_h.insert(d.j2);
            circuits.push(new_h);
        }
    }
}

pub fn part_1(input: String, max: i32) -> u64 {
    let junctions = parse(input);
    let distances = get_distances(&junctions);
    let mut circuits: Vec<HashSet<Junction>> = Vec::new();

    let mut counter = 0;

    for d in distances {
        if counter >= max {
            break;
        }
        counter += 1;

        let h1 = find_hashset_index(&d.j1, &circuits);
        let h2 = find_hashset_index(&d.j2, &circuits);

        associate_to_circuits(&d, h1, h2, &mut circuits);
    }

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));

    let r1 = circuits.get(0).unwrap().len();
    let r2 = circuits.get(1).unwrap().len();
    let r3 = circuits.get(2).unwrap().len();

    (r1 * r2 * r3) as u64
}


pub fn part_2(input: String, max: i32) -> Option<u64> {
    let junctions = parse(input);
    let distances = get_distances(&junctions);
    let mut circuits: Vec<HashSet<Junction>> = Vec::new();

    for d in distances {
        let h1 = find_hashset_index(&d.j1, &circuits);
        let h2 = find_hashset_index(&d.j2, &circuits);

        associate_to_circuits(&d, h1, h2, &mut circuits);

        if circuits[0].len() == max as usize {
            return Some((d.j1.x * d.j2.x) as u64);
        }
    }
    None

}
