pub mod days;
use std::env;
use std::fs;

pub fn read_file(day: i32) -> String {
    let cwd = env::current_dir().unwrap();
    let filename = cwd.join("src/inputs").join(format!("day_{:02}.txt", day));
    fs::read_to_string(filename).unwrap()
}

pub fn read_file_by_full_path(filename: String) -> String {
    fs::read_to_string(filename).unwrap()
}
