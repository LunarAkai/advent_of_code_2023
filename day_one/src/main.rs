use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;
use regex::{Regex};


fn main() {
    day_one_one();
    day_one_two();
}

fn day_one_one() {
    let mut result_one = 0;

    if let Ok(lines) = read_lines("src/input.txt") {
        for line in lines {
            if let Ok(_line) = line {
                let line: &str = _line.as_str();

                let num = return_number_from_line(line);
                result_one = add_two_numbers(result_one, num);
            }
        }
    }

    println!("Day 1, 1: {:?}", result_one);
}

fn day_one_two() {
    let mut result_two = 0;

    if let Ok(lines) = read_lines("src/input.txt") {
        for line in lines {
            if let Ok(_line) = line {
                let line: &str = _line.as_str();
                let pre = get_numbers_from_string(line);
                let pres = pre.as_ref();
                let num = return_number_from_line(pres);

                result_two = add_two_numbers(result_two, num);
            }
        }
    }

    println!("Day 1, 2: {:?}", result_two);

}

fn return_number_from_line(line: &str) -> i32 {
    let re: Regex = Regex::new(r"[0-9]").unwrap();
    let line: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();

    let first_entry_in_vec = find_first_element_in_vec(line.clone());
    let last_entry_in_vec =  find_last_element_in_vec(line.clone());

    let number_string: String = first_entry_in_vec.to_owned() + last_entry_in_vec;

    let number: i32 = number_string.parse().unwrap();

    return number;
}

fn get_numbers_from_string(string_number: &str) -> String {
    let replacement = [
        ("one", "o1e"),
        ("two", "t2o"),
        ("three","t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];

    let mut res = String::from(string_number);
    for(word, replacement) in &replacement {
        res = res.replace(word, replacement);
    }
    return res;
}

fn find_first_element_in_vec(vec: Vec<&str>) -> &str {
    return vec.first().unwrap().trim();
}

fn find_last_element_in_vec(vec: Vec<&str>) -> &str {
    return vec.last().unwrap().trim();
}

fn add_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}

// from: rust-by-example
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::{add_two_numbers, get_numbers_from_string, return_number_from_line};

    #[test]
    fn test_number_from_line() {
        assert_eq!(return_number_from_line("219"), 29);
        assert_eq!(return_number_from_line("8wo3"), 83);
        assert_eq!(return_number_from_line("abc123xyz"), 13);
        assert_eq!(return_number_from_line("x2ne34"), 24);
        assert_eq!(return_number_from_line("49872"), 42);
        assert_eq!(return_number_from_line("z1ight234"), 14);
        assert_eq!(return_number_from_line("7pqrst6teen"), 76);


        assert_eq!(return_number_from_line("268"), 28);
        assert_eq!(return_number_from_line("m5"), 55);
        assert_eq!(return_number_from_line("554"), 54);
    }

    #[test]
    fn test_add_two_numbers(){
        assert_eq!(add_two_numbers(29, 83), 112);
    }

    #[test]
    fn test_number_from_string() {

        assert_eq!(get_numbers_from_string("eighthree"),"e8t3e");
        assert_eq!(get_numbers_from_string("sevenine"),"s7n9e");
    }
}
