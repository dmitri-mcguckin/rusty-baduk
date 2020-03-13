mod stone;
mod board;
use stone::*;
use board::*;
//use std::io;

fn main() {
	let mut board = Board::new(19);
	let mut b_score = 0;
	let mut w_score = 0;

	b_score += board.place(3, 3, 'O');
	b_score += board.place(4, 2, 'O');
	//b_score += board.place(5, 3, 'O');

	w_score += board.place(4, 3, '@');

	b_score += board.place(4, 4, 'O');

    board.draw();
	println!("Black score: {}", b_score);
	println!("White score: {}", w_score);

    /*let mut stopper = String::new();
    let mut x_coord = String::new();
    let mut y_coord = String::new();
    let mut stone_colour = String::new();

    let mut finished: bool = false;
    let quit = "quit";


    while !finished {

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
            //finished = true;
        }

        //Reading in X coord ------------------------------------------
        //figure out how to get a single character out of this 
        println!("Enter in a Letter A-S: ");
        match io::stdin().read_line(&mut x_coord) {
            Ok(_) => {
                println!("success input is: {}", stone_colour);
            },
            Err(e) => println!("error in readin: {}", e)
        }

        //figure out how to do integer inputs
        
        //reading in stone colour ----------------------------------------
        println!("Enter in @ for a black stone or O for a white stone: ");
        match io::stdin().read_line(&mut stone_colour) {
            Ok(_) => {
                println!("success input is: {}", stone_colour);
            },
            Err(e) => println!("error in readin: {}", e)
        }

        //place function call here 

            finished = true;

    }*/

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




/*
	let mut board = Board::new(19);
	board.place('A', 6, '@');
	board.place('C', 1, 'O');
	board.place('D', 2, '@');
	board.place('G', 5, 'O');
	board.place('H', 4, '@');
	board.place('J', 1, 'O');
	board.place('A', 10, '@');
	board.place('A', 1, 'O');
	board.draw();
    let option = board.place('A', 29, 'O');
    let option2 = board.place('A', 19, 'O');
    println!("Option returned: {:?}", option);
    println!("Option2 returned: {:?}", option2);
    board.draw();*/
