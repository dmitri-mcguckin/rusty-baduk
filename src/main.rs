mod stone;
mod board;
use stone::*;
use board::*;

fn main() {
	let mut board = Board::new(19);
	board.place(1, 6, '@');
	board.place(3, 1, 'O');
	board.place(1, 2, '@');
	board.place(1, 5, 'O');
	board.place(4, 4, '@');
	board.place(1, 1, 'O');
	board.place(10, 10, '@');
	board.draw();
}
