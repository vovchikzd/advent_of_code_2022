use std::{io, io::BufRead};

fn main() {
    let mut priority: i32 = 0;
    for line in io::stdin().lock().lines() {
        let clear_line = line.unwrap();
        let half = clear_line.len() / 2;
        let mut lhs = Vec::new();
        let mut rhs = Vec::new();
        for ind in 0..clear_line.len() {
            let symbol = clear_line.chars().nth(ind).unwrap() as u8;
            let to_push: u8;
            match symbol {
                b'a'..=b'z' => to_push = symbol - 96,
                b'A'..=b'Z' => to_push = symbol - 38,
                _ => unreachable!("ERROR"),
            }
            if ind < half {
                lhs.push(to_push);
            } else if ind >= half {
                rhs.push(to_push);
            }
        }
        lhs.sort(); rhs.sort();
        search_same(&mut lhs, &mut rhs, &mut priority);
    }
    println!("{}", priority);
}

fn search_same(lhs: &mut Vec<u8>, rhs: &mut Vec<u8>, priority: &mut i32) {
    while lhs[0] != rhs[0] {
        if lhs[0] < rhs[0] {
            lhs.remove(0);
        } else {
            rhs.remove(0);
        }
    }
    *priority += lhs[0] as i32;
}
