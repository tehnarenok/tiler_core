use std::vec;

use best_macros::public_struct;
#[cfg(feature = "wasm")]
use {
    serde::{Serialize, Deserialize}
};

use crate::{field::{Field}, player::{PlayerState}};

#[public_struct]
#[derive(Clone)]
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize))]
pub struct Game {
    field: Field,
    players: Vec<PlayerState>,
    current_player: usize,
    count_colors: usize,
    seed: [u8; 32],
    steps: Vec<(usize, usize)>,
    is_game_over: bool,
    winner: Option<usize>,
}

impl Game {
    pub fn new(
        field_type: usize,
        count_colors: usize,
        field_size: usize,
        start_cells_type: usize,
        seed: Option<[u8; 32]>,
        current_player: Option<usize>,
        steps: Option<Vec<(usize, usize)>>,
        field: Option<Field>,
        players: Option<Vec<PlayerState>>,
    ) -> Game {
        let field = match field {
            Some(field) => field,
            None => Field::new(field_type, field_size, count_colors, start_cells_type, seed).unwrap()
        };

        let seed = field.seed.clone();

        let players = match players {
            Some(players) => players,
            None => {
                let mut players: Vec<PlayerState> = vec![];

                for start_cells in &field.starts_cells {
                    let mut front: Vec<usize> = vec![];
                    let mut points: usize = 0;
                    let mut color: usize = 0;

                    for start_cell in start_cells {
                        color = field.data[*start_cell].color;
                        points += 1;
                        front.append(&mut field.data[*start_cell].neighbors.clone());
                    }
                    
                    players.push(PlayerState { points: points, front: front, color: color, start_cells: start_cells.clone() })
                }

                players
            }
        };

        Game {
            field: field,
            players: players,
            current_player: match current_player {
                Some(current_player) => current_player,
                None => 0
            },
            count_colors: count_colors,
            seed: seed,
            steps: match steps {
                Some(steps) => steps,
                None => vec![]
            },
            is_game_over: false,
            winner: None,
        }
    }

    pub fn from_game(
        field: Field,
        players: Vec<PlayerState>,
        current_player: usize,
        count_colors: usize,
        seed: [u8; 32],
        steps: Vec<(usize, usize)>
    ) -> Game {
        Game { 
            field: field, 
            players: players, 
            current_player: current_player, 
            count_colors: count_colors, 
            seed: seed, 
            steps: steps,
            is_game_over: false,
            winner: None,
        }
    }
}

impl Game {
    pub fn step(&self, color: usize) -> Game {
        let mut game = self.clone();
        let current_player = game.current_player;
        let mut points = game.players[current_player].points;
        let mut new_front: Vec<usize> = vec![];
        let mut queue: Vec<usize> = game.players[current_player].front.clone();

        while queue.len() > 0 {
            let item = queue.pop();
            
            match item {
                Some(item) => {
                    if game.field.data[item].owner == None && game.field.data[item].color == color {
                        points += 1;
                        game.field.data[item].owner = Some(current_player);
                        queue.append(&mut game.field.data[item].neighbors.clone())
                    } else if game.field.data[item].owner == None && !new_front.contains(&item) {
                        new_front.push(item)
                    }
                },
                None => {
                    break;
                }
            }
        }

        game.players[current_player].front = new_front;
        game.players[current_player].color = color;
        game.players[current_player].points = points;
        game.current_player = (current_player + 1) % 2;
        game.steps.push((current_player, color));

        game.is_game_over = game.get_is_game_over();

        if game.is_game_over {
            game.winner = self.get_winner();
        }

        game
    }
}

impl Game {
    fn get_is_game_over(&self) -> bool {
        for player in &self.players {
            if self.field.data.len() / 2 <= player.points {
                return true
            }
        }

        false
    }

    fn get_winner(&self) -> Option<usize> {
        if !self.is_game_over {
            return None
        }

        let mut winner: Option<usize> = None;
        let mut max_points: usize = 0;

        for (idx, player) in self.players.iter().enumerate() {
            if max_points < player.points {
                max_points = player.points;
                winner = Some(idx)
            }
        }

        winner
    }
}