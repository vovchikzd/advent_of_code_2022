use std::{io, io::BufRead};

fn main() {
    let mut maxes = [0_i32; 3];
    let mut local_max: i32 = 0;
    for line in io::stdin().lock().lines() {
        let clear_line = line.unwrap();
        match &clear_line as &str {
            "" => {
                if local_max > maxes[0] {
                    maxes[0] = local_max;
                    maxes.sort();
                }
                local_max = 0;
            }
            _ => local_max += clear_line.parse::<i32>().unwrap(),
        }
    }
    if local_max > maxes[0] {
        maxes[0] = local_max;
        maxes.sort();
    }
    println!("{}", maxes.iter().sum::<i32>());
}
