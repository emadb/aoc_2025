use aoc_2025::{
    days::day_01::{part_1, part_2},
    read_file,
};

#[test]
fn part_1_test_fake() {
    let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#
        .to_string();
    let res = part_1(input);
    assert_eq!(res, 3);
}

#[test]
fn part_1_test_real() {
    let input = read_file(1);
    let res = part_1(input);
    assert_eq!(res, 1029);
}

#[test]
fn part_2_test_fake() {
    let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#
        .to_string();

    let res = part_2(input);
    assert_eq!(res, 6);
}

#[test]
fn part_2_test_real() {
    let input = read_file(1);
    let res = part_2(input);
    assert_eq!(res, 5892);
}
