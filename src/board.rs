pub struct Board {
    board_string: [char; 9],
}

impl Board {
    pub fn print(&self) {
        for y in 0..3 {
            for x in 0..3 {
                print!("{} ", self.board_string[y * 3 + x])
            }
            println!();
        }
    }

    pub fn new() -> Self {
        Self {
            board_string: ['.'; 9],
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
