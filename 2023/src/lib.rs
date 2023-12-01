use std::fs::read_to_string;

pub fn get_input(day: &str) -> String {
    read_to_string("./inputs/".to_owned() + day).expect("Doesn't exist")
}

pub fn get_lines(day: &str) -> Vec<String>{
    get_input(day)
        .lines()
        .map(String::from)
        .collect()
}
