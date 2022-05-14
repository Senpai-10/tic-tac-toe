use crate::skip_fail;
use std::io::Write;
pub struct Game {
    board: Vec<Vec<String>>,
    turn: String,
    quit: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: vec![
                vec![String::from("1"), String::from("2"), String::from("3")],
                vec![String::from("4"), String::from("5"), String::from("6")],
                vec![String::from("7"), String::from("8"), String::from("9")],
            ],
            turn: String::from("x"),
            quit: false,
        }
    }

    pub fn run(mut self) {
        while !self.quit {
            self.clear_screen();
            println!();
            self.display();
            let input: u8 = skip_fail!(self.input(&format!("{}'s Turn ", self.turn)).parse());

            println!("input: {}", input);
            self.next_turn();
        }
    }

    fn display(&self) {
        for row in self.board.iter() {
            for item in row {
                print!("  {}  ", item);
            }
            print!("\n");
        }
    }

    fn is_board_full(&self) {
        todo!()
    }
    /// check if selected cell is filled by another player
    /// for example: 1 2 3 is x 2 3
    /// o can't fill 1
    fn is_cell_filled(&self) {
        todo!()
    }
    fn find_winner(&self) {
        todo!()
    }

    fn next_turn(&mut self) {
        if self.turn == "x" {
            self.turn = "o".to_string()
        } else {
            self.turn = "x".to_string()
        }
    }

    fn input(&self, message: &str) -> String {
        println!();
        print!("{}", message);
        let mut ret = String::new();
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut ret)
            .expect("Failed to read from stdin");
        return ret.trim().to_string();
    }

    fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }
}
