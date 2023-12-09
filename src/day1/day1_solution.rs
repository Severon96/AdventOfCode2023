use std::fs;

pub fn execute() {
    let contents = fs::read_to_string("/src/day/input.txt");
    println!("{:?}", contents);
}
