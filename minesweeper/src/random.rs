use std::collections::HashSet;

#[cfg(not(target_family = "wasm"))]
use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::*;

use crate::minesweeper::Position;

#[cfg(not(target_family = "wasm"))]
pub fn random_range(min: usize, max: usize) -> usize {
    let mut rng = thread_rng();

    let n: usize = rng.gen_range(min..max);

    n
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

#[cfg(target_family = "wasm")]
pub fn random_range(min: usize, max: usize) -> usize {
    (random() * (max - min) as f64).floor() as usize + min
}

pub fn calculate_mines(width: usize, height: usize, mine_count: usize) -> HashSet<Position> {
    let mut mines = HashSet::new();

    while mines.len() < mine_count {
        mines.insert((random_range(0, width), random_range(0, height)));
    }
    mines
}
