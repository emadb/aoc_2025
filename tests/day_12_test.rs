use aoc_2025::{
    days::{day_12::part_1},
    read_file,
};

#[test]
fn part_1_test_fake() {

    let input =
r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"#
        .to_string();
    let res = part_1(input);
    assert_eq!(res, 1);
}

#[test]
fn part_1_test_real() {

    let input = read_file(12);
    let res = part_1(input);
    assert_eq!(res, 569);
}
