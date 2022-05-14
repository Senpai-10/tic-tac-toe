mod game;
mod skip_fail;

use game::Game;

fn main() {
    let mut game = Game::new();

    game.run();
}
