use aoc_2025::{
    days::day_05::{part_1, part_2},
    read_file,
};

#[test]
fn part_1_test_fake() {
    let input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#
    .to_string();
    let res = part_1(input);
    assert_eq!(res, 3);
}

#[test]
fn part_1_test_real() {
    let input = read_file(5);
    let res = part_1(input);
    assert_eq!(res, 782);
}

#[test]
fn part_2_test_fake() {
    let input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#
    .to_string();
    let res = part_2(input);
    assert_eq!(res, 14);
}

#[test]
fn part_2_test_real() {
    let input = read_file(5);
    let res = part_2(input);
    assert_eq!(res, 353863745078671);
}
