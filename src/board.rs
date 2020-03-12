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

    //A-S and 1-19
    pub fn place(& mut self, x: char, y: usize, color: char) -> Option<bool> { 
        //check if x is in the bounds else error throw
        if y > 19 || y < 1 
        {
            let option = None;
            return option;
        }
        let valid = check_bounds(x);
        if valid == false
        {
            return None;
        }
        let s = self.stones.get_mut(y - 1)
                               .unwrap()
                               .get_mut(((x as u8) - 65) as usize)
                               .unwrap();
        s.place(color);
        Some(true)
    }
}


    pub fn check_bounds(x_coord: char) -> bool{
        let mut valid: bool = false;
        let set = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S'];
        for value in set{
            if value == x_coord {
                valid = true;
                return valid;
            }
        }
        valid
    }
