use crate::game::Game;

use super::try_step;

pub fn step(game: &Game) -> Game {
    let game = game.clone();

    let players_colors: Vec<usize> = game.players.iter().map(|el| el.color).collect();

    let mut best_color: Option<usize> = None;
    let mut best_diff: Option<usize> = None;

    for color in 0..game.count_colors {
        if players_colors.contains(&color) {
            continue;
        }

        let result = try_step(&game, color);

        if let Some((_, diff)) = result {
            if let Some(best_diff) = best_diff  {
                if diff < best_diff {
                    continue;
                }
            }

            best_diff = Some(diff);
            best_color = Some(color);
        }
    }

    if let Some(best_color) = best_color {
        let game = game.step(best_color).unwrap();

        game
    } else {
        super::easy::step(&game)
    }
}