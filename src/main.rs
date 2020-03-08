mod stone;
mod board;
use stone::*;
use board::*;

fn main() {
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
    board.draw();

}

#[test]
fn out_of_bounds(){
    let mut board = Board::new(19);
    assert_eq!(board.place('A', 20, '@'), None);
}
