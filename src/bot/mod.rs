#[cfg(feature = "serde")]
use {
    crate::locale::LocaleExport,
};

use crate::{game::Game, locale::Locale};

mod easy;
mod medium;

fn try_step(game: &Game, color: usize) -> Option<(Game, usize)> {
    let player = game.current_player;
    let points = game.players[player].points;
    let game = game.step(color);

    match game {
        Ok(game) => {
            let diff = game.players[player].points - points;

            Some((game, diff))
        },
        _ => {
            None
        }
    }

    
}

struct BotType<'a> {
    title: Locale<'a>,
    step: fn(game: &Game) -> Game 
}

const BOT_TYPES: &'static [BotType<'static>] = &[
    BotType {
        title: Locale { 
            ru: "Простой", 
            en: "Easy",
        },
        step: easy::step,
    },
    BotType {
        title: Locale { 
            ru: "Средний", 
            en: "Medium",
        },
        step: medium::step,
    },
];

#[cfg(feature = "serde")]
pub fn get_bot_types() -> Vec<LocaleExport> {
    BOT_TYPES.iter().map(|el| el.title.export()).collect()
}

pub fn step(bot_type: usize, game: Game) -> Game {
    if bot_type > BOT_TYPES.len() - 1 {
        panic!("Cannot find bot with id {}. Last id is {}, '{}'", bot_type, BOT_TYPES.len() - 1, BOT_TYPES.last().unwrap().title.en);
    }

    let step = BOT_TYPES[bot_type].step;
    step(&game)
}