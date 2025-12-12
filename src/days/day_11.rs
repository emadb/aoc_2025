use std::collections::HashMap;

fn parse(input: String) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let (device, outs_str) = line.split_once(":").unwrap();
        let outs = outs_str.split(" ")
            .map(|o| o.trim().to_string())
            .filter(|o| o != "")
            .collect();
        map.insert(device.trim().to_string(), outs);
    }
    map
}


fn get_key(s: String, k: String) -> String {
    format!("{}-{}", s.clone(), k.clone())
}

fn count_paths_rec(device: String, dest: String, map: &HashMap<String, Vec<String>>, cache: &mut HashMap<String, i64>) -> i64 {
    if device == dest {
        return 1
    } else {
        let mut count = 0;

        match map.get(&device) {
            Some(outs) => {
                for out in outs {
                    let k = get_key(out.to_string(), dest.to_string()).to_string();
                    match cache.get(&k) {
                        Some(step) => count +=step,
                        None => {
                            let step = count_paths_rec(out.clone(), dest.clone(), map, cache);
                            cache.insert(k, step);
                            count += step;
                        }
                    }
                }
            },
            None => {}
        }
        count
    }
}



pub fn part_1(input: String) -> i64 {
    let map = parse(input);
     let mut cache: HashMap<String, i64> = HashMap::new();
    count_paths_rec("you".to_string(), "out".to_string(), &map, &mut cache)
}


pub fn part_2(input: String) -> i64 {
    let map = parse(input);
    let mut cache: HashMap<String, i64> = HashMap::new();
    let p1 = count_paths_rec("svr".to_string(), "dac".to_string(), &map, &mut cache);
    let p2 = count_paths_rec("dac".to_string(), "fft".to_string(), &map, &mut cache);
    let p3 = count_paths_rec("fft".to_string(), "out".to_string(), &map, &mut cache);

    let p4 = count_paths_rec("svr".to_string(), "fft".to_string(), &map, &mut cache);
    let p5 = count_paths_rec("fft".to_string(), "dac".to_string(), &map, &mut cache);
    let p6 = count_paths_rec("dac".to_string(), "out".to_string(), &map, &mut cache);

    (p1 * p2 * p3) + (p4 * p5 * p6)

}