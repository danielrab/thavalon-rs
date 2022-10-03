use std::env;

fn main() {
    let player_names: Vec<String> = env::args().skip(1).collect();
    let game = match thavalon::Game::new(player_names, thavalon::ShuffleMode::Full) {
        Ok(game) => game,
        Err(thavalon::Error::InvalidPlayerCount(count)) => panic!("can't start a game with {count} players")
    };
    println!("{game:#?}")
}