use std::{io, io::BufRead};

fn main() {
    let mut score: i32 = 0;
    for line in io::stdin().lock().lines() {
        let clear_line = line.unwrap();
        if &clear_line as &str == "" {
            continue;
        }
        let enemy = clear_line.chars().nth(0).unwrap();
        let myne = clear_line.chars().nth(2).unwrap();
        match enemy {
            'A' => {
                match myne {
                    'X' => score += 4,
                    'Y' => score += 8,
                    'Z' => score += 3,
                    _   => unreachable!("You shouldn't be here"),
                }
            }

            'B' => {
                match myne {
                    'X' => score += 1,
                    'Y' => score += 5,
                    'Z' => score += 9,
                    _   => unreachable!("You shouldn't be here"),
                }
            } 

            'C' => {
                match myne {
                    'X' => score += 7,
                    'Y' => score += 2,
                    'Z' => score += 6,
                    _   => unreachable!("You shouldn't be here"),
                }
            } 

            _ => unreachable!("You shouldn't be here"),
        }
    }
    println!("{}", score);
}
