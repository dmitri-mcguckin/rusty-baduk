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
}
