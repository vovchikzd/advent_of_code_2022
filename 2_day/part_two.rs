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
                    'X' => score += 3,
                    'Y' => score += 4,
                    'Z' => score += 8,
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
                    'X' => score += 2,
                    'Y' => score += 6,
                    'Z' => score += 7,
                    _   => unreachable!("You shouldn't be here"),
                }
            } 

            _ => unreachable!("You shouldn't be here"),
        }
    }
    println!("{}", score);
}
