use std::{io, io::BufRead};

fn main() {
    let mut max: i32 = 0;
    let mut local_max: i32 = 0;
    for line in io::stdin().lock().lines() {
        let clear_line = line.unwrap();
        match &clear_line as &str {
            "" => {
                if local_max > max {
                    max = local_max;
                }
                local_max = 0;
            }
            _ => local_max += clear_line.parse::<i32>().unwrap(),
        }
    }
    println!("{}", max);
}
