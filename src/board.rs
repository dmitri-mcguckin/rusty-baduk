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

    pub fn place(& mut self, x: char, y: usize, color: char) {
        let s = self.stones.get_mut(y - 1)
                               .unwrap()
                               .get_mut(((x as u8) - 65) as usize)
                               .unwrap();
        s.place(color);
    }
}
