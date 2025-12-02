use aoc_2025::{
    days::day_02::{part_1, part_2},
    read_file,
};

#[test]
fn part_1_test_fake() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
        .to_string();
    let res = part_1(input);
    assert_eq!(res, 1227775554);
}

#[test]
fn part_1_test_real() {
    let input = read_file(2);
    let res = part_1(input);
    assert_eq!(res, 40055209690);
}

#[test]
fn part_2_test_fake() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
        .to_string();
    let res = part_2(input);
    assert_eq!(res, 4174379265);
}

#[test]
fn part_2_test_real() {
    let input = read_file(2);
    let res = part_2(input);
    assert_eq!(res, 50857215650);
}
