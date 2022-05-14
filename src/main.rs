mod game;
mod skip_fail;

use game::Game;

fn main() {
    let game = Game::new();

    game.run();
}
