use best_macros::public_struct;
#[cfg(feature = "serde")]
use {
    serde::{Serialize, Deserialize}
};

#[public_struct]
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct PlayerState {
    points: usize,
    front: Vec<usize>,
    color: usize,
    start_cells: Vec<usize>
}