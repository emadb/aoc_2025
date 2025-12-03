use aoc_2025::{
    days::day_03::{part_1, part_2},
    read_file,
};

#[test]
fn part_1_test_fake() {
    let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#
        .to_string();
    let res = part_1(input);
    assert_eq!(res, 357);
}

#[test]
fn part_1_test_real() {
    let input = read_file(3);
    let res = part_1(input);
    assert_eq!(res, 17408);
}
#[test]
fn part_2_test_fake() {
    let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#
        .to_string();
    let res = part_2(input);
    assert_eq!(res, 3121910778619);
}

#[test]
fn part_2_test_real() {
    let input = read_file(3);
    let res = part_2(input);
    assert_eq!(res, 172740584266849);
}
