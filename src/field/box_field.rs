use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use crate::field::{Size, GenerateResult, Cell, Coords, CellSize, COLORS};
use crate::locale::Locale;

pub const STRART_CELLS: &'static [Locale<'static>] = &[
    Locale {
        ru: "Диагональ",
        en: "Diagonal"
    },
    Locale {
        ru: "Стороны",
        en: "Side-toside"
    },
    Locale {
        ru: "Meeting",
        en: "Meeting"
    },
    Locale {
        ru: "Random 1",
        en: "Random 1"
    },
];

pub fn generate_fn<'a>(size: Size, start_points_type: usize, count_colors: usize, seed: [u8; 32]) -> GenerateResult {
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

    for i in 0..size.x {
        for j in 0..size.y {
            let mut neighbors: Vec<usize> = vec![];
            if i > 0 { neighbors.push(((i - 1) * size.y + j) as usize); }
            if i < size.x - 1 { neighbors.push(((i + 1) * size.y + j) as usize); }
            if j > 0 { neighbors.push((i * size.y + j - 1) as usize); }
            if j < size.y - 1 { neighbors.push((i * size.y + j + 1) as usize); }

            let cell = Cell {
                coord: Coords {
                    x: (1.0 / size.x as f32) * i as f32,
                    y: (1.0 / size.y as f32) * j as f32,
                },
                owner: None,
                color: (r.next_u32() % count_colors as u32) as usize,
                size: CellSize {
                    x: (1.0 / size.x as f32),
                    y: (1.0 / size.y as f32),
                },
                neighbors: neighbors
            };

            data.push(cell);
        }
    }

    let mut start_cells: Vec<Vec<usize>> = vec![];

    match start_points_type {
        0 => {
            start_cells.push(vec![0 as usize]);
            start_cells.push(vec![(size.x * size.y - 1) as usize]);
        },
        1 => {
            start_cells.push(vec![0 as usize, (size.y * (size.x - 1)) as usize]);
            start_cells.push(vec![(size.y - 1) as usize, (size.x * size.y - 1) as usize]);
        },
        2 => {
            start_cells.push(vec![0 as usize, (size.x * size.y - 1) as usize]);
            start_cells.push(vec![(size.y - 1) as usize, (size.y * (size.x - 1)) as usize]);
        }
        _ => {
            start_cells.push(vec![0 as usize]);
            start_cells.push(vec![(size.x * size.y - 1) as usize]);
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
        size: size.clone()
    }
}