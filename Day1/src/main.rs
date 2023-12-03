use std::fs::read_to_string;

fn get_input() -> Option<String> {
    let read_str = read_to_string("/home/alire/personal/Advent-of-Code/Day1/input.txt");
    match read_str {
       Ok(string) => return Some(string),
       Err(_) => return None,
    }
}

fn extract_digits(line: &str) -> String { 
    let mut digits = String::new();
    // get the line and check for digit that comes first in the iterator
    for char in line.chars() { 
        if char.is_numeric() {
            digits.push(char);
            break;
        }
    }
    for char in line.chars().rev() {
        if char.is_numeric() { 
            digits.push(char);
            break;
        }
    }

    digits
}



fn main() { 
    let mut sum_of_calibration = 0;
    let input = get_input();
    if input.is_some() { 
        for line in input.unwrap()
            .lines() {
                let int_result = extract_digits(line).parse::<i32>();
                if int_result.is_ok() { 
                    sum_of_calibration += int_result.unwrap();
                }
            }
    }
    println!("{}", sum_of_calibration)
}
