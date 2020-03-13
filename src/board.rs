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
        let valid = check_bounds(x, y);
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


    pub fn check_bounds(x_coord: char, size: usize) -> bool{
        let mut valid: bool = false;
        let mut set = vec![];
        if size == 19{
            set.push('A');set.push('B');set.push('C');set.push('D');set.push('E');set.push('F');set.push('G');set.push('H');set.push('I');
            set.push('J');set.push('K');set.push('L');set.push('M');
            set.push('N');set.push('O');set.push('P');set.push('Q');set.push('R'); set.push('S');
        }
        else if size == 13{
            set.push('A');set.push('B');set.push('C');set.push('D');set.push('E');set.push('F');set.push('G');set.push('H');set.push('I');
            set.push('J');set.push('K');set.push('L');set.push('M');
        }
        else if size == 9{
            set.push('A');set.push('B');set.push('C');set.push('D');set.push('E');set.push('F');set.push('G');set.push('H');set.push('I');
        }
        for value in set{
            if value == x_coord {
                valid = true;
                return valid;
            }
        }
        valid
    }
