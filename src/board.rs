use crate::*;

pub struct Board {
    size: usize,
    stones: Vec<Vec<stone::Stone>>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        let mut rows = Vec::new();
        for _ in 0..size {
            let mut cols = Vec::new();
            for _ in 0..size {
                cols.push(Stone::new('+'));
            }
            rows.push(cols);
        }

        Self { size: size, stones: rows }
    }

    pub fn draw(& self) {
        println!("{}x{} Board:", self.size, self.size);

        for x in 0..self.size { print!("{} ", ((x + 65) as u8) as char); }
        println!("");

        for x in 0..self.size {
            for y in 0..self.size {
                print!("{} ", self.stones.get(x).unwrap().get(y).unwrap().color());
            }
            println!(" {}", x + 1);
        }
    }

	fn is_valid(& self, x: usize, y: usize) -> bool {
		x > 0 && x <= self.size && y > 0 && y <= self.size
	}

	fn liberties(& self, x: usize, y: usize) -> usize {
		let mut liberties = 0;
		if self.is_valid(x - 1, y) && self.stones.get(x - 1).unwrap().get(y).unwrap().color() == '+' { liberties += 1; }
		if self.is_valid(x + 1, y) && self.stones.get(x + 1).unwrap().get(y).unwrap().color() == '+' { liberties += 1; }
		if self.is_valid(x, y - 1) && self.stones.get(x).unwrap().get(y - 1).unwrap().color() == '+' { liberties += 1; }
		if self.is_valid(x, y + 1) && self.stones.get(x).unwrap().get(y + 1).unwrap().color() == '+' { liberties += 1; }
		liberties
	}

	fn check_dead(& mut self, x: usize, y: usize, color: char) -> usize {
		let mut score = 0;
		if self.is_valid(x, y) {
			let s = self.stones.get(x).unwrap().get(y).unwrap();
			if s.color() == color && self.liberties(x, y) == 0 {
				score += self.check_dead(x - 1, y, color);
				score += self.check_dead(x + 1, y, color);
				score += self.check_dead(x, y - 1, color);
				score += self.check_dead(x, y + 1, color);
				self.place(x + 1, y + 1, '+');
				score += 1;
			}
		}
		score
	}

    //A-S and 1-19
    pub fn place(& mut self, x: usize, y: usize, color: char) -> usize { 
		// Offset
		let x = x - 1;
		let y = y - 1;

		// Check bounds
        if !self.is_valid(x, y) { return 0; }

		// Place the stone
		self.stones.get_mut(x).unwrap().get_mut(y).unwrap().place(color);

		// Check for dead stones
		let op_color = if color == 'O' { '@' } else { 'O' };
		let mut score = self.check_dead(x - 1, y, op_color);
		score += self.check_dead(x + 1, y, op_color);
		score += self.check_dead(x, y - 1, op_color);
		score += self.check_dead(x, y + 1, op_color);
		score
    }
}
