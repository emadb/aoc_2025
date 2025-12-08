use aoc_2025::{
    days::day_08::{part_1, part_2},
    read_file,
};

#[test]
fn part_1_test_fake() {

    let input =
r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#
        .to_string();
    let res = part_1(input, 10);
    assert_eq!(res, 40);
}



#[test]
fn part_1_test_real() {
    let input = read_file(8);
    let res = part_1(input, 1000);
    assert_eq!(res, 140008);
}

#[test]
fn part_2_test_fake() {

    let input =
r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#
        .to_string();
    let res = part_2(input, 20).unwrap();
    assert_eq!(res, 25272);
}


#[test]
fn part_2_test_real() {
    let input = read_file(8);
    let res = part_2(input, 1000).unwrap();
    assert_eq!(res, 9253260633);
}
