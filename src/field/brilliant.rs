use crate::field::{Cell, CellSize, Coords, GenerateResult, Size, COLORS};
use crate::locale::Locale;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

pub const STRART_CELLS: &'static [Locale<'static>] = &[Locale {
    ru: "Напротив",
    en: "Across",
}];

fn get_line_len(size: &Size, idx: usize) -> usize {
    match idx {
        idx if (idx as i32 * 2 + 1) < size.y => idx * 2 + 1,
        idx if ((size.x - (idx + 1) as i32) * 2 + 1) < size.y => {
            (size.x as usize - (idx + 1)) * 2 + 1
        }
        _ => size.y as usize,
    }
}

fn get_cut_len(size: &Size, idx: usize) -> usize {
    let max_cut_len = ((size.y - 1) / 3 - 2) + 1;

    let half = (size.x - 1) / 2;
    let offset = (idx as i32 - half).abs();

    if offset <= max_cut_len {
        return (max_cut_len - offset) as usize;
    }

    0
}

pub fn generate_fn<'a>(
    size: Size,
    _: usize,
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

    let mut coords: Vec<Size> = vec![];

    for i in 0..size.x {
        let len = get_line_len(&size, i as usize);
        let cut_len = get_cut_len(&size, i as usize);

        for j in 0..len {
            if (j as i32 - (len as i32 - 1) / 2).abs() < cut_len as i32 {
                continue;
            }

            coords.push(Size {
                x: i,
                y: j as i32 + (size.y - len as i32) / 2,
            });

            let cell = Cell {
                coord: Coords {
                    x: (1.0 / size.x as f32) * i as f32,
                    y: (1.0 / size.y as f32) * (j + (size.y as usize - len) / 2) as f32,
                },
                owner: None,
                color: (r.next_u32() % count_colors as u32) as usize,
                size: CellSize {
                    x: (1.0 / size.x as f32),
                    y: (1.0 / size.y as f32),
                },
                neighbors: vec![],
            };

            data.push(cell);
        }
    }

    let count_of_cells = *(&data.len());

    for i in 0..count_of_cells {
        let mut neighbors: Vec<usize> = vec![];

        for j in 0..count_of_cells {
            if coords[i].x - 1 == coords[j].x && coords[i].y == coords[j].y {
                neighbors.push(j);
            }
            if coords[i].x + 1 == coords[j].x && coords[i].y == coords[j].y {
                neighbors.push(j);
            }
            if coords[i].x == coords[j].x && coords[i].y - 1 == coords[j].y {
                neighbors.push(j);
            }
            if coords[i].x == coords[j].x && coords[i].y + 1 == coords[j].y {
                neighbors.push(j);
            }
        }

        data[i].neighbors = neighbors.clone();
    }

    let mut start_cells: Vec<Vec<usize>> = vec![];

    start_cells.push(vec![0 as usize]);
    start_cells.push(vec![data.len() - 1]);

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
