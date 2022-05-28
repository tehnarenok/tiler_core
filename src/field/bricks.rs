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

    let count_in_row_odd = size.x / 2 - 1;
    let count_in_row_even = size.x / 2;

    for i in 0..size.y {
        let (count_in_row, offset) = if i % 2 == 0 {
            (count_in_row_even, 0.0)
        } else {
            (count_in_row_odd, (1.0 / size.x as f32))
        };

        for j in 0..count_in_row {
            let mut neighbors: Vec<usize> = vec![];
            if i < size.y - 1 {
                if j > 0 || offset > 0.0 {
                    neighbors.push(data.len() + count_in_row_even as usize - 1);
                }
                if j < count_in_row - 1 || offset > 0.0 {
                    neighbors.push(data.len() + count_in_row_even as usize);
                }
            }
            if i > 0 {
                if j > 0 {
                    neighbors.push(data.len() - count_in_row_even as usize);
                }
                if j < count_in_row - 1 {
                    neighbors.push(data.len() - count_in_row_even as usize + 1);
                }
            }
            if j > 0 {
                neighbors.push(data.len() - 1 as usize);
            }
            if j < count_in_row - 1 {
                neighbors.push(data.len() + 1 as usize);
            }

            let cell = Cell {
                coord: Coords {
                    x: (2.0 / size.x as f32) * j as f32 + offset,
                    y: (1.0 / size.y as f32) * i as f32,
                },
                owner: None,
                color: (r.next_u32() % count_colors as u32) as usize,
                size: CellSize {
                    x: (2.0 / size.x as f32),
                    y: (1.0 / size.y as f32),
                },
                neighbors: neighbors,
            };

            data.push(cell);
        }
    }

    let mut start_cells: Vec<Vec<usize>> = vec![];

    match start_points_type {
        0 => {
            start_cells.push(vec![0 as usize]);
            start_cells.push(vec![data.len() - 1 as usize]);
        }
        1 => {
            start_cells.push(vec![0 as usize, data.len() - (size.x / 2) as usize]);
            start_cells.push(vec![(size.x / 2 - 1) as usize, (data.len() - 1) as usize]);
        }
        2 => {
            start_cells.push(vec![0 as usize, (data.len() - 1) as usize]);
            start_cells.push(vec![
                (size.x / 2 - 1) as usize,
                data.len() - (size.x / 2) as usize,
            ]);
        }
        _ => {
            start_cells.push(vec![0 as usize]);
            start_cells.push(vec![data.len() - 1 as usize]);
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
