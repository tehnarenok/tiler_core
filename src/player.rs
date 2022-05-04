use best_macros::public_struct;
#[cfg(feature = "wasm")]
use {
    serde::{Serialize, Deserialize}
};

#[public_struct]
#[derive(Clone)]
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize))]
struct PlayerState {
    points: usize,
    front: Vec<usize>,
    color: usize,
    start_cells: Vec<usize>
}