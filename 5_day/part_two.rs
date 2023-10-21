use std::{io, io::BufRead};

fn main() {
    let mut moves = Vec::new();
    let mut crates = Vec::new();

    for line in io::stdin().lock().lines() {
        let string = line.unwrap();
        if string.contains("move") {
            moves.push(string);
        } else {
            crates.push(string);
        }
    }
    crates.pop();

    let mut boxes = Vec::new();
    let last_line = crates[crates.len() - 1].clone();
    for ind in 0..last_line.len() {
        let symbol = last_line.chars().nth(ind).unwrap();

        match symbol as u8 {
            b'0'..=b'9' => {
                let mut stack = Vec::new();
                for i in (0..(crates.len() - 1)).rev() {
                    let to_push = crates[i].chars().nth(ind).unwrap();
                    if to_push != ' ' {
                        stack.push(to_push);
                    }
                }
                boxes.push(stack);
            }
            _ => {}
        }
    }

    for move_line in moves {
        let mut move_nums = Vec::new();
        let mut flag = false;
        let mut str_num = Vec::new();

        for ind in 0..move_line.len() {
            let symbol = move_line.chars().nth(ind).unwrap();
            match symbol as u8 {
                b'0'..=b'9' => str_num.push(symbol),
                b'a'..=b'z' => {}
                b' ' => match flag {
                    false => flag = true,
                    true => {
                        move_nums.push(
                            str_num
                                .clone()
                                .into_iter()
                                .collect::<String>()
                                .parse::<usize>()
                                .unwrap(),
                        );
                        flag = false;
                        str_num.clear();
                    }
                },
                _ => {}
            }
        }
        move_nums.push(
            str_num
                .clone()
                .into_iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap(),
        );

        for mv in (0..move_nums[0]).rev() {
            let index_remove = boxes[move_nums[1] - 1].len() - 1 - mv;
            let to_move = boxes[move_nums[1] - 1][index_remove];
            boxes[move_nums[1] - 1].remove(index_remove);
            boxes[move_nums[2] - 1].push(to_move);
        }
    }

    for test in boxes {
        print!("{}", test[test.len() - 1]);
    }
    println!();
}
