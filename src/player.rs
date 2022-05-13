use best_macros::public_struct;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[public_struct]
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct PlayerState {
    points: usize,
    front: Vec<usize>,
    color: usize,
    start_cells: Vec<usize>,
}
