use crate::game::Game;
use rand::{thread_rng, Rng};

pub fn step(game: &Game) -> Game {
    let game = game.clone();

    let players_colors: Vec<usize> = game.players.iter().map(|el| el.color).collect();

    let mut best_color: usize = (thread_rng().gen::<u32>() % game.count_colors as u32) as usize;

    while players_colors.contains(&best_color) {
        best_color = (thread_rng().gen::<u32>() % game.count_colors as u32) as usize;
        
    }

    let game = game.step(best_color).unwrap();

    game
}