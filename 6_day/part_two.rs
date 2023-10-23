use std::{io, io::BufRead};

fn main() {
    let mut readed = String::new();
    let _ = io::stdin().lock().read_line(&mut readed);
    let mut buffer: Vec<char> = readed.chars().collect();
    buffer.pop();

    let mut local_buffer: Vec<char> = Vec::with_capacity(14);
    let mut counter = 0;

    for ind in 0..buffer.len() {
        while local_buffer.contains(&buffer[ind]) {
            local_buffer.remove(0);
        }

        if local_buffer.len() != 14 {
            local_buffer.push(buffer[ind]);
        } else {
            counter = ind;
            break;
        }
    }

    println!("{}", counter);
}
