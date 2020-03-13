mod board;

#[test]
fn out_of_bounds(){
	let board = board::Board::new(19);
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
