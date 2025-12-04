use aoc_2025::{
    days::day_04::{part_1, part_2},
    read_file,
};

#[test]
fn part_1_test_fake() {
    let input = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#
        .to_string();
    let res = part_1(input);
    assert_eq!(res, 13);
}

#[test]
fn part_1_test_real() {
    let input = read_file(4);
    let res = part_1(input);
    assert_eq!(res, 1486);
}

#[test]
fn part_2_test_fake() {
    let input = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#
        .to_string();
    let res = part_2(input);
    assert_eq!(res, 43);
}

#[test]
fn part_2_test_real() {
    let input = read_file(4);
    let res = part_2(input);
    assert_eq!(res, 9024);
}
