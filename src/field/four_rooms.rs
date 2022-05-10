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
    }
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

    let size_x = size.x * 2 + 1;
    let size_y = size.y * 2 + 1;

    for room in 0..4 {
        let dx = match room {
            1 | 3 => size.x + 1,
            _ => 0 
        };

        let dy = match room {
            2 | 3 => size.y + 1,
            _ => 0
        };

        for i in 0..size.x {
            for j in 0..size.y {
                let mut neighbors: Vec<usize> = vec![];
                let offset  = ((size.x * size.y) * room) as usize;
                if i > 0 { neighbors.push(((i - 1) * size.y + j) as usize + offset); }
                if i < size.x - 1 { neighbors.push(((i + 1) * size.y + j) as usize + offset); }
                if j > 0 { neighbors.push((i * size.y + j - 1) as usize + offset); }
                if j < size.y - 1 { neighbors.push((i * size.y + j + 1) as usize + offset); }

                let cell = Cell {
                    coord: Coords {
                        x: (1.0 / size_x as f32) * (i + dx) as f32,
                        y: (1.0 / size_y as f32) * (j + dy) as f32,
                    },
                    owner: None,
                    color: (r.next_u32() % count_colors as u32) as usize,
                    size: CellSize {
                        x: (1.0 / size_x as f32),
                        y: (1.0 / size_y as f32),
                    },
                    neighbors: neighbors
                };

                data.push(cell);
            }
        }
    }

    let mut start_cells: Vec<Vec<usize>> = vec![];

    match start_points_type {
        1 => {
            start_cells.push(vec![
                (size.x * size.y * 0 + 0) as usize, (size.x * size.y * 0 + size.y * (size.x - 1)) as usize,
                (size.x * size.y * 1 + 0) as usize, (size.x * size.y * 1 + size.y * (size.x - 1)) as usize,
                (size.x * size.y * 2 + 0) as usize, (size.x * size.y * 2 + size.y * (size.x - 1)) as usize,
                (size.x * size.y * 3 + 0) as usize, (size.x * size.y * 3 + size.y * (size.x - 1)) as usize,
            ]);
            start_cells.push(vec![
                (size.x * size.y * 0 + (size.y - 1)) as usize, (size.x * size.y * 0 + size.x * size.y - 1) as usize,
                (size.x * size.y * 1 + (size.y - 1)) as usize, (size.x * size.y * 1 + size.x * size.y - 1) as usize,
                (size.x * size.y * 2 + (size.y - 1)) as usize, (size.x * size.y * 2 + size.x * size.y - 1) as usize,
                (size.x * size.y * 3 + (size.y - 1)) as usize, (size.x * size.y * 3 + size.x * size.y - 1) as usize,
            ]);
        },
        2 => {
            start_cells.push(vec![
                (size.x * size.y * 0 + 0) as usize, (size.x * size.y * 0 + size.x * size.y - 1) as usize,
                (size.x * size.y * 1 + 0) as usize, (size.x * size.y * 1 + size.x * size.y - 1) as usize,
                (size.x * size.y * 2 + 0) as usize, (size.x * size.y * 2 + size.x * size.y - 1) as usize,
                (size.x * size.y * 3 + 0) as usize, (size.x * size.y * 33 + size.x * size.y - 1) as usize,
            ]);
            start_cells.push(vec![
                (size.x * size.y * 0 + (size.y - 1)) as usize, (size.x * size.y * 0 + size.y * (size.x - 1)) as usize,
                (size.x * size.y * 1 + (size.y - 1)) as usize, (size.x * size.y * 1 + size.y * (size.x - 1)) as usize,
                (size.x * size.y * 2 + (size.y - 1)) as usize, (size.x * size.y * 2 + size.y * (size.x - 1)) as usize,
                (size.x * size.y * 3 + (size.y - 1)) as usize, (size.x * size.y * 3 + size.y * (size.x - 1)) as usize,
            ]);
        }
        _ => {
            start_cells.push(vec![
                (size.x * size.y * 0) as usize,
                (size.x * size.y * 1) as usize,
                (size.x * size.y * 2) as usize,
                (size.x * size.y * 3) as usize,
            ]);
            start_cells.push(vec![
                (size.x * size.y * 1 - 1) as usize,
                (size.x * size.y * 2 - 1) as usize,
                (size.x * size.y * 3 - 1) as usize,
                (size.x * size.y * 4 - 1) as usize,
            ]);
        },
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
        size: Size {x: size_x, y: size_y},
    }
}