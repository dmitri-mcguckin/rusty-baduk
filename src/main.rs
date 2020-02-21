mod stone;
mod board;
use stone::*;
use board::*;

fn main() {
	let mut s1 = Stone::new('O', (0,0));
	let mut s2 = Stone::new('@', (1,1));
	s1.display();
	s2.display();
}
