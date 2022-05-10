mod box_field;
mod four_rooms;

use best_macros::*;
use rand::{thread_rng, Rng, SeedableRng, RngCore};
extern crate std;
use rand_chacha::ChaCha20Rng;
use serde::{Serialize, Deserialize};
use crate::locale::Locale;
#[cfg(feature = "serde")]
use {
    crate::locale::LocaleExport,
};

pub const COLORS: [&str; 11] = [
    "#026d35",
    "#5b23a4",
    "#05f0e4",
    "#e30000",
    "#ff3ac2",
    "#fd791a",
    "#1c9bfd",
    "#dde026",
    "#08b35c",
    "#b8b8b8",
    "#bb92fc",
];

#[public_struct]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Size {
    x: i32,
    y: i32,
}

#[public_struct]
#[derive(Debug, Serialize, Deserialize)]
struct GenerateResult {
    data: Vec<Cell>,
    start_cells: Vec<Vec<usize>>,
    size: Size
}

#[public_struct]
#[derive(Debug)]
pub struct FieldType<'a> {
    title: Locale<'a>,
    sizes: &'a [Size],
    start_cells: &'a [Locale<'a>],
    generate_fn: fn(size: Size, start_points_type: usize, count_colors: usize, seed: [u8; 32]) -> GenerateResult
}

#[public_struct]
#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize)]
struct FieldTypeExport {
    title: LocaleExport,
    sizes: Vec<Size>,
    start_cells: Vec<LocaleExport>
}

#[cfg(feature = "serde")]
impl<'a> FieldType<'a> {
    pub fn export(&self) -> FieldTypeExport {
        FieldTypeExport { 
            title: self.title.export(), 
            sizes: self.sizes.to_vec(), 
            start_cells: self.start_cells.into_iter().map(|el| el.export()).collect()
        }
    }
}

pub const FIELD_TYPES: &'static [FieldType<'static>] = &[
    FieldType {
        title: Locale {
            ru: "Базовое поле",
            en: "Basic field"
        },
        sizes: &[
            Size { x: 15, y: 9},
            Size { x: 25, y: 15},
            Size { x: 40, y: 24},
            Size { x: 50, y: 30},
            Size { x: 75, y: 45},
        ],
        start_cells: box_field::STRART_CELLS,
        generate_fn: box_field::generate_fn,
    },
    FieldType {
        title: Locale {
            ru: "Четыре комнаты",
            en: "Four rooms"
        },
        sizes: &[
            Size { x: 15, y: 9},
            Size { x: 21, y: 11},
            Size { x: 29, y: 17},
        ],
        start_cells: four_rooms::STRART_CELLS,
        generate_fn: four_rooms::generate_fn,
    },
];

#[cfg(feature = "serde")]
pub fn get_fields_types() -> Vec<FieldTypeExport> {
    FIELD_TYPES.into_iter().map(|el| el.export()).collect()
}

#[public_struct]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Coords {
    x: f32,
    y: f32,
}

#[public_struct]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CellSize {
    x: f32,
    y: f32,
}

#[public_struct]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Cell {
    coord: Coords,
    owner: Option<usize>,
    color: usize,
    size: CellSize,
    neighbors: Vec<usize>
}

#[public_struct]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Field {
    data: Vec<Cell>,
    starts_cells: Vec<Vec<usize>>,
    seed: [u8; 32],
    field_type: usize,
    count_colors: usize,
    start_cells_type: usize,
    field_size: Size,
}

impl Field {
    pub fn new(
        field_type: usize,
        field_size: usize,
        count_colors: usize,
        start_cells_type: usize,
        seed: Option<[u8; 32]>,
    ) -> Option<Self> {
        let seed: [u8; 32] = match seed {
            Some(seed) => seed,
            None => {
                let seed = thread_rng().gen::<[u8; 32]>();
                seed
            }
        };

        let mut field: Field = Self {
            field_type,
            count_colors,
            field_size: Size {
                x: 10,
                y: 10,
            },
            data: vec![],
            starts_cells: vec![],
            seed,
            start_cells_type,
        };

        if field_type >= FIELD_TYPES.len() {
            field.field_type = 0 as usize;
        }
        let field_type = FIELD_TYPES.get(field.field_type);
        match field_type {
            Some(field_type) => {
                let sizes = field_type.sizes;
                let field_size_idx = match field_size >= sizes.len() {
                    true => 0,
                    false => field_size
                };

                let size = sizes.get(field_size_idx);
                match size {
                    Some(size) => {
                        field.field_size = *size;
                    },
                    None => return None
                }
            },
            None => return None
        }

        let field_type = FIELD_TYPES.get(field.field_type);
        match field_type {
            Some(field_type) => {
                let fun = field_type.generate_fn;
                let gen_result = fun(
                    field.field_size,
                    field.start_cells_type,
                    field.count_colors,
                    field.seed
                );
                field.data = gen_result.data;
                field.starts_cells = gen_result.start_cells;
                field.field_size = gen_result.size;

                let mut r: ChaCha20Rng = SeedableRng::from_seed(seed);

                let mut players_colors = [(r.next_u32() % count_colors as u32) as usize, (r.next_u32() % count_colors as u32) as usize];

                while players_colors[0] == players_colors[1] {
                    players_colors[1] = (r.next_u32() % count_colors as u32) as usize;
                }

                for (idx, start_cells) in field.starts_cells.iter().enumerate() {
                    for start_cell in start_cells {
                        field.data[*start_cell].color = players_colors[idx]
                    }
                }

                for start_cells in &field.starts_cells {
                    let mut front: Vec<usize> = vec![];

                    for start_cell in start_cells {
                        front.append(&mut field.data[*start_cell].neighbors.clone());
                    }

                    for cell in front {
                        while field.data[cell].color == players_colors[0] || field.data[cell].color == players_colors[1] {
                            field.data[cell].color = (r.next_u32() % count_colors as u32) as usize;
                        }
                    }
                }
            },
            None => {}
        }

        Some(field)
    }
}