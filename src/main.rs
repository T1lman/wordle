mod api;
mod game;
mod words;
fn main() {
    let mut game = game::Game::new();
    game.play();
}
