use std::{io, io::BufRead};

fn main() {
    let lines = io::stdin().lock().lines();
    let mut trees: Vec<Vec<i32>> = Vec::new();
    let mut visible: i32;
    for line in lines {
        let string = line.unwrap();
        let mut to_push: Vec<i32> = Vec::new();
        for ch in string.chars() {
            to_push.push(ch.to_string().parse::<i32>().unwrap());
        }
        trees.push(to_push);
    }

    visible = (trees.len() * 2 + (trees[0].len() - 2) * 2) as i32;

    for row_ind in 1..(trees.len() - 1) {
        for column_ind in 1..(trees[row_ind].len() - 1) {
            let score = check_visibility(row_ind, column_ind, &trees);
            if score > visible {
                visible = score;
            }
        }
    }

    println!("{}", visible);
}

fn check_visibility(row: usize, column: usize, trees: &Vec<Vec<i32>>) -> i32 {
    let cheking = trees[row][column];

    let mut score_up = 0;
    let mut score_down = 0;
    let mut score_left = 0;
    let mut score_right = 0;

    // check up visibility
    for row_ind in (0..row).rev() {
        score_up += 1;
        if trees[row_ind][column] >= cheking {
            break;
        }
    }

    //check down
    for row_ind in (row + 1)..trees.len() {
        score_down += 1;
        if trees[row_ind][column] >= cheking {
            break;
        }
    }

    // check left
    for col_ind in (0..column).rev() {
        score_left += 1;
        if trees[row][col_ind] >= cheking {
            break;
        }
    }

    // check right
    for col_ind in (column + 1)..trees[row].len() {
        score_right += 1;
        if trees[row][col_ind] >= cheking {
            break;
        }
    }

    // let to_return = 
    score_up * score_down * score_left * score_right
}
