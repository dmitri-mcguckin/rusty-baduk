mod stone;
mod board;
use stone::*;
use board::*;

fn main() {
    //make game loop and input system 
	let mut board = Board::new(19);
    board.draw();

    let mut finished = false;

    while !finished {
        //if <input> enter coords and switch to other colour also clear and redraw the screen
        //if <input> make finished = false
        finished = true;
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
    let mut board = Board::new(19);
    assert_eq!(check_bounds('A'), true);
    assert_eq!(check_bounds('Z'), false);
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
