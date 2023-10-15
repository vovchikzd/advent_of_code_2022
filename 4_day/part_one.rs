use std::{io, io::BufRead};

// _ => local_max += clear_line.parse::<i32>().unwrap(),
fn main() {
    let mut counter: i32 = 0;
    let mut lines = Vec::new();

    for line in io::stdin().lock().lines() {
        lines.push(line.unwrap());
    }

    for line in lines {
        let mut numbers = Vec::new();
        let mut str_number = Vec::new();
        // print!("{:?}\t", line);
        for ind in 0..line.len() {
            let symbol = line.chars().nth(ind).unwrap();
            match symbol as u8 {
                b'0'..=b'9' => str_number.push(symbol),
                b'-' => {
                    numbers.push(
                        str_number
                            .clone()
                            .into_iter()
                            .collect::<String>()
                            .parse::<i32>()
                            .unwrap(),
                    );
                    str_number.clear();
                }
                b',' => {
                    numbers.push(
                        str_number
                            .clone()
                            .into_iter()
                            .collect::<String>()
                            .parse::<i32>()
                            .unwrap(),
                    );
                    str_number.clear();
                }
                _ => unreachable!("{}", symbol),
            }
        }
        numbers.push(
            str_number
                .clone()
                .into_iter()
                .collect::<String>()
                .parse::<i32>()
                .unwrap(),
        );
        // println!("{:?}", numbers);
        if numbers[0] <= numbers[2] && numbers[1] >= numbers[3] {
            counter += 1;
        } else if numbers[2] <= numbers[0] && numbers[3] >= numbers[1] {
            counter += 1;
        }
    }

    println!("{}", counter);
}
