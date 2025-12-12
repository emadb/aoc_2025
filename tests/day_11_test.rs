use aoc_2025::{
    days::{day_11::part_1, day_11::part_2},
    read_file,
};

#[test]
fn part_1_test_fake() {

    let input =
r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#
        .to_string();
    let res = part_1(input);
    assert_eq!(res, 5);
}


#[test]
fn part_1_test_real() {

    let input = read_file(11);
    let res = part_1(input);
    assert_eq!(res, 746);
}


#[test]
fn part_2_test_fake() {

    let input =
r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#
        .to_string();
    let res = part_2(input);
    assert_eq!(res, 2);
}



#[test]
fn part_2_test_real() {

    let input = read_file(11);
    let res = part_2(input);
    assert_eq!(res, 370500293582760);
}
