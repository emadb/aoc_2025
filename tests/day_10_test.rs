use aoc_2025::{
    days::{day_10::part_1, day_10::part_2},
    read_file,
};

#[test]
fn part_1_test_fake() {

    let input =
r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#
        .to_string();
    let res = part_1(input);
    assert_eq!(res, 7);
}

#[test]
fn part_1_test_real() {
    let input = read_file(10);
    let res = part_1(input);
    assert_eq!(res, 486);
}

#[test]
fn part_2_test_fake() {

    let input =
r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#
        .to_string();
    let res = part_2(input);
    assert_eq!(res, 33);
}


#[test]
fn part_2_test_real() {
    let input = read_file(10);
    let res = part_2(input);
    assert_eq!(res, 486);
}