use std::{io, io::BufRead};

fn main() {
    let mut priority: i32 = 0;
    let mut lines = Vec::new();
    for line in io::stdin().lock().lines() {
        let clear_line = line.unwrap();
        lines.push(clear_line);
    }
    while lines.len() >= 3 {
        let mut first = Vec::new();
        let mut second = Vec::new();
        let mut third = Vec::new();

        for ind in 0..lines[2].len() {
            let symbol = lines[2].chars().nth(ind).unwrap() as u8;
            match symbol {
                b'a'..=b'z' => third.push(symbol - 96),
                b'A'..=b'Z' => third.push(symbol - 38),
                _ => unreachable!("{}", symbol),
            }
        }
        lines.remove(2);

        for ind in 0..lines[1].len() {
            let symbol = lines[1].chars().nth(ind).unwrap() as u8;
            match symbol {
                b'a'..=b'z' => second.push(symbol - 96),
                b'A'..=b'Z' => second.push(symbol - 38),
                _ => unreachable!("{}", symbol),
            }
        }
        lines.remove(1);

        for ind in 0..lines[0].len() {
            let symbol = lines[0].chars().nth(ind).unwrap() as u8;
            match symbol {
                b'a'..=b'z' => first.push(symbol - 96),
                b'A'..=b'Z' => first.push(symbol - 38),
                _ => unreachable!("{}", symbol),
            }
        }
        lines.remove(0);

        first.sort();
        second.sort();
        third.sort();
        search_same(&mut first, &mut second, &mut third, &mut priority);
    }
    println!("{}", priority);
}

fn search_same(first: &mut Vec<u8>, second: &mut Vec<u8>, third: &mut Vec<u8>, priority: &mut i32) {
    while first[0] != second[0] || second[0] != third[0] {
        while first[0] != second[0] {
            if first[0] < second[0] {
                first.remove(0);
            } else {
                second.remove(0);
            }
        }

        while second[0] != third[0] {
            if second[0] < third[0] {
                second.remove(0);
            } else {
                third.remove(0);
            }
        }
    }
    *priority += first[0] as i32;
}
