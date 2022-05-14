pub struct Game {
    board: Vec<Vec<String>>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: vec![
                vec![String::from("1"), String::from("2"), String::from("3")],
                vec![String::from("4"), String::from("5"), String::from("6")],
                vec![String::from("7"), String::from("8"), String::from("9")],
            ],
        }
    }

    pub fn run(self) {
        self.display();
    }

    fn display(&self) {
        for row in self.board.iter() {
            for item in row {
                print!("  {}  ", item);
            }
            print!("\n");
        }
    }

    fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }
}
