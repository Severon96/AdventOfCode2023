use std::collections::HashMap;
use std::fs;
use std::str::Split;

pub fn execute() {
    let contents_result = fs::read_to_string("input_day1.txt");
    let contents = contents_result.unwrap();
    let lines = contents.split("\n");

    process_lines(lines);
}

fn process_lines(lines: Split<&str>) {

    for line in lines {
        println!("{}", line);

        process_line();
    }
}

fn process_line() {
    let mut numbers = setup_numbers();



    for (number_int, number_string) in numbers.iter() {
        println!("{} : {}", number_int, number_string);
    }
}

fn find_number_index(line: &str, number: char) -> isize {
    for (index, character) in line.chars().enumerate() {
        if character == number {
            return index as isize;
        }
    }
    -1
}

fn setup_numbers() -> HashMap<i8, &'static str> {
    let mut numbers: HashMap<i8, &str> = HashMap::new();

    numbers.insert(0, "zero");
    numbers.insert(1, "one");
    numbers.insert(2, "two");
    numbers.insert(3, "three");
    numbers.insert(4, "four");
    numbers.insert(5, "five");
    numbers.insert(6, "six");
    numbers.insert(7, "seven");
    numbers.insert(8, "eight");
    numbers.insert(9, "nine");

    return numbers;
}
