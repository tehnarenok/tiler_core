use crate::game::Game;

use super::try_step;

pub fn step(game: &Game) -> Game {
    let game = game.clone();

    let players_colors: Vec<usize> = game.players.iter().map(|el| el.color).collect();

    let mut best_color: usize = 0;
    let mut best_diff: usize = 0;

    for color in 0..game.count_colors {
        if players_colors.contains(&color) {
            continue;
        }

        let (_, diff) = try_step(&game, color);

        if diff > best_diff {
            best_diff = diff;
            best_color = color;
        }
    }

    let game = game.step(best_color).unwrap();

    game
}