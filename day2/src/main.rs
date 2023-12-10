use std::fs::read_to_string;

fn get_input() -> Option<String> {
    let read_str = read_to_string("/home/alire/personal/Advent-of-Code/day2/test-input.txt");
    match read_str {
       Ok(string) => return Some(string),
       Err(_) => return None,
    }
}

enum Color {
   Blue,
   Green,
   Red,
}

// takes number and game from a result
fn take_number(Game: &str) { 
    if Game.starts_with("Game") { 

    }
}

fn extract_games(game_line: &str) -> String { 
    let game_id = game_line.chars().nth(5);
    let game_result: Vec<&str> = game_line.split(";").collect();
    if game_id.is_some() { 
        let game_id = game_id.unwrap();
    } else {
        panic!("This is the bad part");
    }
    let total_green = 0;
    let total_red = 0;
    let total_blue = 0;
    for round in game_result { 
        match take_number(round) {
           "green" => { }
        }
    }
    println!("{:?}", game_result);
    todo!()
}



fn main() { 
    let mut sum_of_calibration = 0;
    let input = get_input();
    if input.is_some() { 
        for line in input.unwrap()
            .lines() {
                let int_result = extract_games(line).parse::<i32>();
                if int_result.is_ok() { 
                    sum_of_calibration += int_result.unwrap();
                }
            }
    }
    println!("{}", sum_of_calibration)
}
