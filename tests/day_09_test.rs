use aoc_2025::{
    days::{day_09::part_2, day_09::part_1},
    read_file,
};

#[test]
fn part_1_test_fake() {

    let input =
r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#
        .to_string();
    let res = part_1(input);
    assert_eq!(res, 50);
}

#[test]
fn part_1_test_real() {
    let input = read_file(9);
    let res = part_1(input);
    assert_eq!(res, 4748826374);
}


#[test]
fn part_2_test_fake() {

    let input =
r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#
        .to_string();
    let res = part_2(input);
    assert_eq!(res, 24);
}


#[test]
fn part_2_test_real() {
    let input = read_file(9);
    let res = part_2(input);
    assert_eq!(res, 1554370486);
}
