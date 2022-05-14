// TODO rewrite code

use crate::skip_fail;
use std::io::Read;
use std::io::Write;

pub struct Game {
    board: Vec<Vec<String>>,
    turn: String,
    players_moves: Players,
    quit: bool,
}

struct Players {
    x: Vec<u8>,
    o: Vec<u8>,
}
// TODO handle draw
// TODO add colors to x and o chars
impl Game {
    pub fn new() -> Game {
        Game {
            board: vec![
                vec![1.to_string(), 2.to_string(), 3.to_string()],
                vec![4.to_string(), 5.to_string(), 6.to_string()],
                vec![7.to_string(), 8.to_string(), 9.to_string()],
            ],
            turn: String::from("x"),
            players_moves: Players {
                x: vec![0, 0, 0, 0, 0, 0, 0, 0],
                o: vec![0, 0, 0, 0, 0, 0, 0, 0],
            },
            quit: false,
        }
    }

    pub fn run(mut self) {
        while !self.quit {
            self.clear_screen();
            let winner = self.find_winner();

            if winner.is_empty() == false {
                println!("{} has won!!!!", winner);
                self.pause();
                self.reset();
            }

            self.clear_screen();

            // println!("x moves: {:?}", self.players_moves.x);
            // println!("o moves: {:?}", self.players_moves.o);
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

    fn fill_cell(&mut self, cell_number: u8, turn: String) {
        let player = match turn.as_str() {
            "x" => &mut self.players_moves.x,
            "o" => &mut self.players_moves.o,
            _ => &mut self.players_moves.x,
        };

        for (row_index, row) in self.board.clone().iter().enumerate() {
            for (col_index, item) in row.iter().enumerate() {
                if item == &cell_number.to_string() {
                    self.board[row_index][col_index] = turn.to_string();
                    player[row_index] += 1;
                    player[col_index + 3] += 1;
                    if row_index == col_index {
                        player[6] += 1;
                    }
                    if row_index == 2 - col_index {
                        player[7] += 1;
                    }
                }
            }
        }
    }

    fn find_winner(&self) -> String {
        let mut winner = String::new();

        for i in 0..8 {
            if self.players_moves.x[i] == 3 {
                winner = "x".to_string();
            } else if self.players_moves.o[i] == 3 {
                winner = "o".to_string();
            }
        }

        return winner;
    }

    fn reset(&mut self) {
        self.board = vec![
            vec![1.to_string(), 2.to_string(), 3.to_string()],
            vec![4.to_string(), 5.to_string(), 6.to_string()],
            vec![7.to_string(), 8.to_string(), 9.to_string()],
        ];
        self.turn = String::from("x");
        self.players_moves = Players {
            x: vec![0, 0, 0, 0, 0, 0, 0, 0],
            o: vec![0, 0, 0, 0, 0, 0, 0, 0],
        };
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

    // source: https://users.rust-lang.org/t/rusts-equivalent-of-cs-system-pause/4494/4
    fn pause(&self) {
        let mut stdin = std::io::stdin();
        let mut stdout = std::io::stdout();

        // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
        write!(stdout, "Press any key to continue...").unwrap();
        stdout.flush().unwrap();

        // Read a single byte and discard
        let _ = stdin.read(&mut [0u8]).unwrap();
    }
}
