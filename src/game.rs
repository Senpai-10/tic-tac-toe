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
                vec![1.to_string(), 2.to_string(), 3.to_string()],
                vec![4.to_string(), 5.to_string(), 6.to_string()],
                vec![7.to_string(), 8.to_string(), 9.to_string()],
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
            self.fill_cell(input, self.turn.to_owned());
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

    fn fill_cell(&mut self, cell_number: u8, turn: String) {
        // self.find_winner();

        for (row_index, row) in self.board.clone().iter().enumerate() {
            for (item_index, item) in row.iter().enumerate() {
                if item == &cell_number.to_string() {
                    self.board[row_index][item_index] = turn.to_string();
                }
            }
        }
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
