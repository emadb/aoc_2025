use aoc_2025::{
    days::day_06::{part_1, part_2},
    read_file,
};

#[test]
fn part_1_test_fake() {
    let input = r#"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "#
        .to_string();
    let res = part_1(input);
    assert_eq!(res, 4277556);
}

#[test]
fn part_1_test_real() {
    let input = read_file(6).to_string();
    let res = part_1(input);
    assert_eq!(res, 6343365546996);
}

#[test]
fn part_2_test_fake() {
    let input = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#
        .to_string();
    let res = part_2(input);
    assert_eq!(res, 3263827);
}

#[test]
fn part_2_test_real() {
    let input = read_file(6).to_string();
    let res = part_2(input);
    assert_eq!(res, 11136895955912);
}
