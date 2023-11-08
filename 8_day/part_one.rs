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
            if check_visibility(row_ind, column_ind, &trees) {
                visible += 1;
            }
        }
    }

    println!("{}", visible);
}

fn check_visibility(row: usize, column: usize, trees: &Vec<Vec<i32>>) -> bool {
    let cheking = trees[row][column];

    let mut flag_up = true;
    let mut flag_down = true;
    let mut flag_left = true;
    let mut flag_right = true;

    // check up visibility
    for row_ind in 0..row {
        if trees[row_ind][column] >= cheking {
            flag_up = false;
            break;
        }
    }

    //check down
    for row_ind in (row + 1)..trees.len() {
        if trees[row_ind][column] >= cheking {
            flag_down = false;
            break;
        }
    }

    // check left
    for col_ind in 0..column {
        if trees[row][col_ind] >= cheking {
            flag_left = false;
            break;
        }
    }

    // check right
    for col_ind in (column + 1)..trees[row].len() {
        if trees[row][col_ind] >= cheking {
            flag_right = false;
            break;
        }
    }

    // let to_return = 
    flag_up || flag_down || flag_left || flag_right
}
