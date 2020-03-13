mod stone;
mod board;
use stone::*;
use board::*;
use std::io;

fn main() {
	let mut board = Board::new(19);
	let mut b_score = 0;
	let mut w_score = 0;

    let mut stopper = String::new();
    let mut x_coord = String::new();
    let mut y_coord = String::new();
    let mut stone_colour = 'O';

    let mut finished: bool = false;
    let quit = "quit";


    while !finished {
		// Draw the board
		board.draw();

        //Reading in X coord ------------------------------------------
        //figure out how to get a single character out of this 
        println!("Enter in x coordinate [0-9]: ");
        match io::stdin().read_line(&mut x_coord) {
            Ok(_) => {
                println!("success input is: {}", x_coord);
            },
            Err(e) => println!("error in readin: {}", e)
        }

        //Reading in Y coord ------------------------------------------
        //figure out how to get a single character out of this 
        println!("Enter in y coordinate [A-Z]: ");
        match io::stdin().read_line(&mut y_coord) {
            Ok(_) => {
                println!("success input is: {}", y_coord);
            },
            Err(e) => println!("error in readin: {}", e)
        }
        
        // Convert the args
		let x = x_coord.trim().parse::<usize>().unwrap();
		let y = (y_coord.chars().next().unwrap() as u32) as usize - 64;
		println!("---- ({}, {})", x, y);
		
		// Place the stone
		let score = board.place(x, y, stone_colour);

		println!("Black score: {}", b_score);
		println!("White score: {}", w_score);

        //reading in stopping variable ---------------------------
        println!("if you wish to quit type 'quit': ");
        match io::stdin().read_line(&mut stopper) {
            Ok(_) => {
                println!("success input is: {}", stopper);
            },
            Err(e) => println!("error in readin: {}", e)
        }

        if stopper.eq("quit") == true{ //does not work
            println!("\n\nHEY WE DID IT YO\n\n");
            finished = true;
        }
		
		// Switch turns
		if stone_colour == 'O' {
			b_score += score;
			stone_colour = '@'
		}
		else {
			w_score += score;
			stone_colour = 'O'
		}
    }

}

#[test]
fn out_of_bounds(){
    let mut board = Board::new(19);
    assert_eq!(board.place('A', 20, '@'), None);
}

#[test]
fn in_bounds(){
    let mut board = Board::new(19);
    assert_eq!(board.place('A', 19, '@'), Some(true));
}

#[test]
fn check_bounds_test(){
    let mut board = Board::new(9);
    assert_eq!(check_bounds('A', 9), true);
    assert_eq!(check_bounds('Z', 9), false);
}
