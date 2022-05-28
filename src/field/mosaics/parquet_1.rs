use crate::field::{Cell, CellSize, Coords, GenerateResult, Size, COLORS};
use crate::locale::Locale;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

pub const STRART_CELLS: &'static [Locale<'static>] = &[
    Locale {
        ru: "Диагональ",
        en: "Diagonal",
    },
    Locale {
        ru: "Стороны",
        en: "Side-toside",
    },
    Locale {
        ru: "Meeting",
        en: "Meeting",
    },
];

pub fn generate_fn<'a>(
    size: Size,
    start_points_type: usize,
    count_colors: usize,
    seed: [u8; 32],
) -> GenerateResult {
    let mut r: ChaCha20Rng = SeedableRng::from_seed(seed);
    let mut data: Vec<Cell> = vec![];
    let count_colors = match count_colors >= 4 {
        true => count_colors,
        false => 4,
    };

    let count_colors = match count_colors < COLORS.len() {
        true => count_colors,
        false => COLORS.len() - 1,
    };

    let mut count_pairs = 0;

    for i in 0..(size.x / 2) {
        for j in 0..(size.y / 2) {
            let order = if (i + j) % 2 == 0 { true } else { false };

            let mut neighbors_first: Vec<usize> = vec![];
            let mut neighbors_second: Vec<usize> = vec![];

            if order {
                if j > 0 {
                    neighbors_first.push((count_pairs - 1) * 2 + 1);
                    neighbors_second.push((count_pairs - 1) * 2 + 1);
                }

                if j < (size.y / 2 - 1) {
                    neighbors_first.push((count_pairs + 1) * 2);
                    neighbors_second.push((count_pairs + 1) * 2);
                }

                if i > 0 {
                    neighbors_first.push((count_pairs - size.y as usize / 2) * 2);
                    neighbors_first.push((count_pairs - size.y as usize / 2) * 2 + 1);
                }

                if i < (size.x / 2 - 1) {
                    neighbors_second.push((count_pairs + size.y as usize / 2) * 2);
                    neighbors_second.push((count_pairs + size.y as usize / 2) * 2 + 1);
                }
            } else {
                if j > 0 {
                    neighbors_first.push((count_pairs - 1) * 2);
                    neighbors_first.push((count_pairs - 1) * 2 + 1);
                }

                if j < (size.y / 2 - 1) {
                    neighbors_second.push((count_pairs + 1) * 2);
                    neighbors_second.push((count_pairs + 1) * 2 + 1);
                }

                if i > 0 {
                    neighbors_first.push((count_pairs - size.y as usize / 2) * 2 + 1);
                    neighbors_second.push((count_pairs - size.y as usize / 2) * 2 + 1);
                }

                if i < (size.x / 2 - 1) {
                    neighbors_first.push((count_pairs + size.y as usize / 2) * 2);
                    neighbors_second.push((count_pairs + size.y as usize / 2) * 2);
                }
            }

            neighbors_first.push(count_pairs * 2 + 1);
            neighbors_second.push(count_pairs * 2);

            data.push(Cell {
                coord: Coords {
                    x: (1.0 / size.x as f32) * (i * 2) as f32,
                    y: (1.0 / size.y as f32) * (j * 2) as f32,
                },
                owner: None,
                color: (r.next_u32() % count_colors as u32) as usize,
                size: CellSize {
                    x: (if order { 1.0 } else { 2.0 } / size.x as f32),
                    y: (if order { 2.0 } else { 1.0 } / size.y as f32),
                },
                neighbors: neighbors_first,
            });

            data.push(Cell {
                coord: Coords {
                    x: (1.0 / size.x as f32) * (i * 2 + if order { 1 } else { 0 }) as f32,
                    y: (1.0 / size.y as f32) * (j * 2 + if order { 0 } else { 1 }) as f32,
                },
                owner: None,
                color: (r.next_u32() % count_colors as u32) as usize,
                size: CellSize {
                    x: (if order { 1.0 } else { 2.0 } / size.x as f32),
                    y: (if order { 2.0 } else { 1.0 } / size.y as f32),
                },
                neighbors: neighbors_second,
            });

            count_pairs += 1;
        }
    }

    let mut start_cells: Vec<Vec<usize>> = vec![];

    match start_points_type {
        0 => {
            start_cells.push(vec![0]);
            start_cells.push(vec![data.len() - 1]);
        }
        1 => {
            start_cells.push(vec![0 as usize, size.y as usize - 1]);
            start_cells.push(vec![data.len() - size.y as usize, data.len() - 1]);
        }
        2 => {
            start_cells.push(vec![0 as usize, data.len() - 1]);
            start_cells.push(vec![size.y as usize - 1, data.len() - size.y as usize]);
        }
        _ => {
            start_cells.push(vec![0 as usize]);
            start_cells.push(vec![data.len() - 1]);
        }
    }

    for (idx, item) in start_cells.iter().enumerate() {
        for point in item {
            let mut cell = data[*point].clone();
            cell.owner = Some(idx);
            data[*point] = cell;
        }
    }

    GenerateResult {
        data: data,
        start_cells: start_cells,
        size: size.clone(),
    }
}
