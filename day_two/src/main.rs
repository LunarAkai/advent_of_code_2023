use std::collections::LinkedList;
use std::fs::File;
use std::io;
use std::io::{BufRead, empty};
use std::path::Path;
use std::str::SplitN;


// Day 2, 1: Threshold (max):   - 12 red cubes,
//                              - 13 green cubes,
//                              - 14 blue cubes?

fn main() {
    day_two_one();
    //get_individual_cube_shows(" 1 red, 5 blue, 1 green; 16 blue, 3 red; 6 blue, 5 red; 4 red, 7 blue, 1 green", 3);
}

fn day_two_one() {
    let id_sum: i32 = 0;

    if let Ok(lines) = read_lines("src/input.txt") {
        for line in lines {
            if let Ok(_line) = line {
                let line: &str = _line.as_str();

                get_game_id(line);

                let line_right_side = split_string_one_side(line, ":", false);

                let counter = count_cube_shows(line_right_side);

                let individual_cube_shows = get_individual_cube_shows(line_right_side, counter);


            }
        }
    }
}

fn get_game_id(line_string: &str) -> i32 {
    let mut string = split_string_one_side(line_string, ":", true);

    let edit = string.replace("Game ", "");

    return edit.parse().unwrap();
}

fn count_cube_shows(line_string: &str) -> i32 {
    let mut result = 0;

    for c in line_string.chars() {
        if c.eq(&';') {
            result += 1;
        }
    }
    return result;
}

fn get_individual_cube_shows(line_string: &str, counter: i32) -> LinkedList<&str>{
    let mut strings: LinkedList<&str> = Default::default();
    let mut new_line_string = line_string; // why? weird mut stuff idk


    for c in 0..counter {

        let mut result;
        if c == counter - 1 {
            //new_line_string = split_string_one_side(new_line_string, "; ", true);
            result = split_string_both_sides(new_line_string, "\n");
        } else {
            result = split_string_both_sides(new_line_string, "; ");
            new_line_string = split_string_one_side(new_line_string, "; ", false);
        }

        strings.push_back(result.next().unwrap());
    }

    println!("{:?}", strings);
    return strings;
}

fn is_game_possible_with_limit(red: i32, blue: i32, green: i32, limit_r: i32, limit_g: i32, limit_b: i32) -> bool {
    return red <= limit_r && green <= limit_g && blue <= limit_b
}

fn split_string_one_side<'a>(string: &'a str, split_where: &'a str, left: bool) -> &'a str  {
    let mut split = string.splitn(2, split_where);
    return match left {
        true => {
            split.next().unwrap()
        }
        false => {
            split.next().unwrap();
            split.next().unwrap()
        }
    }
}

fn split_string_both_sides<'a>(string: &'a str, split_where: &'a str) -> SplitN<'a, &'a str> {
    return string.splitn(2, split_where).into_iter()
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
    use crate::{count_cube_shows, get_game_id, split_string_one_side};

    #[test]
    fn test_get_game_id() {
        assert_eq!(get_game_id("Game 1: 1 red, 5 blue, 1 green; 16 blue, 3 red; 6 blue, 5 red; 4 red, 7 blue, 1 green"), 1);
        assert_eq!(get_game_id("Game 26: 1 green, 4 blue, 17 red; 15 red, 3 green, 3 blue; 2 blue, 2 red; 18 red, 2 green, 11 blue; 6 red, 7 blue; 10 blue, 1 green, 4 red"), 26);
    }

    #[test]
    fn test_count_cube_shows() {
        assert_eq!(count_cube_shows("Game 1: 1 red, 5 blue, 1 green; 16 blue, 3 red; 6 blue, 5 red; 4 red, 7 blue, 1 green"), 3);
        assert_eq!(count_cube_shows("Game 26: 1 green, 4 blue, 17 red; 15 red, 3 green, 3 blue; 2 blue, 2 red; 18 red, 2 green, 11 blue; 6 red, 7 blue; 10 blue, 1 green, 4 red"), 5);
    }

    #[test]
    fn test_split_right() {
        assert_eq!(split_string_one_side("Game 1: 1 red, 5 blue, 1 green; 16 blue, 3 red; 6 blue, 5 red; 4 red, 7 blue, 1 green", ":", false), " 1 red, 5 blue, 1 green; 16 blue, 3 red; 6 blue, 5 red; 4 red, 7 blue, 1 green");
    }
}