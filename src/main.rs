mod board;
mod stone;
use board::*;
use std::io;
use stone::*;

fn main() {
    let mut board = Board::new(19);
    let mut b_score = 0;
    let mut w_score = 0;
    let mut pass = 0;

    let mut stone_colour = '@';

    let mut finished: bool = false;

    // Draw the board
    board.draw();

    while !finished {
        let mut stopper = String::new();
        let mut x_coord = String::new();
        let mut y_coord = String::new();

        // Switch turns
        if stone_colour == 'O' {
            println!("White to play!");
            stone_colour = '@'
        } else {
            println!("Black to play!");
            stone_colour = 'O'
        }

        //reading in pass variable ---------------------------
        println!("Pass? [Y/N]: ");
        match io::stdin().read_line(&mut stopper) {
            Ok(_) => {}
            Err(e) => println!("error in readin: {}", e),
        }

        stopper = stopper.trim().to_string();

        if stopper.to_uppercase().eq("YES") || stopper.to_uppercase().eq("Y") {
            pass += 1;

            // Determine if the game ended
            finished = pass == 2;

            continue;
        } else {
            pass = 0;
        }

        //Reading in Y coord ------------------------------------------
        //figure out how to get a single character out of this
        println!("Enter in x coordinate [char]: ");
        match io::stdin().read_line(&mut y_coord) {
            Ok(_) => {}
            Err(e) => println!("error in readin: {}", e),
        }

        //Reading in X coord ------------------------------------------
        //figure out how to get a single character out of this
        println!("Enter in y coordinate [int]: ");
        match io::stdin().read_line(&mut x_coord) {
            Ok(_) => {}
            Err(e) => println!("error in readin: {}", e),
        }

        // Convert the args
        let x = x_coord.trim().parse::<usize>().unwrap();
        let y = (y_coord.chars().next().unwrap() as u32) as usize - 64;

        // Place the stone
        let score = board.place(x, y, stone_colour);

        // Recalculate the score
        if stone_colour == 'O' {
            b_score += score;
        } else {
            w_score += score;
        }

        println!("Black score: {}", b_score);
        println!("White score: {}", w_score);

        // Draw the board
        board.draw();
    }

    println!("Game ended:");
    println!("\tBlack score: {}", b_score);
    println!("\tWhite score: {}", w_score);
}

#[test]
fn out_of_bounds() {
    let mut board = Board::new(19);
    assert_eq!(board.place('A' as u16 as usize, 20, '@') as i16, 0);
}

#[test]
fn in_bounds() {
    let mut board = Board::new(19);
    assert_eq!(board.place('A' as u16 as usize, 19, '@'), 0);
}

#[test]
fn check_bounds_test() {
    let mut board = Board::new(19);

    // Valid coords
    assert_eq!(board.is_valid('A' as i16 - 65, 1), true);
    assert_eq!(board.is_valid('A' as i16 - 65, 18), true);
    assert_eq!(board.is_valid('S' as i16 - 65, 1), true);
    assert_eq!(board.is_valid('S' as i16 - 65, 18), true);

    // Invalid coords
    assert_eq!(board.is_valid('Z' as i16, 1), false);
}
