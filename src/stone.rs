

mod stone{
pub struct Stone{
    colour: char,//for now it is a char, later we can do somethign with it
    pos: (u64, u64), 

}


impl Stone{
    //do we want any struct vars for how many are dead for scoring?
    
    pub fn new(& mut self, color: char, coord: (u64,u64)) -> &Self {
        self.colour = color;
        self.pos.0 = coord.0;
        self.pos.1 = coord.1;
        self
    }

    //other heckin funcs 
}

}
