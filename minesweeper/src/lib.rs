mod minesweeper;
mod random;

use std::cell::RefCell;

use minesweeper::*;
use wasm_bindgen::prelude::*;

thread_local! {
    static MINESWEEPER: RefCell<Minesweeper> = RefCell::new(Minesweeper::new(10, 10, 15));
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello {}", name));
}

#[wasm_bindgen(js_name = createMinesweeper)]
pub fn create_minesweeper(x: usize, y: usize, nb: usize) {
    MINESWEEPER.with(|ms| ms.replace(Minesweeper::new(x, y, nb)));
}

#[wasm_bindgen(js_name = getState)]
pub fn get_state() -> String {
    MINESWEEPER.with(|ms| ms.borrow().to_string())
}

#[wasm_bindgen(js_name = openField)]
pub fn open_field(x: usize, y: usize) {
    MINESWEEPER.with(|ms| ms.borrow_mut().open((x, y)));
}

#[wasm_bindgen(js_name = toggleFlag)]
pub fn toggle_flag(x: usize, y: usize) {
    MINESWEEPER.with(|ms| ms.borrow_mut().toggle_flag((x, y)));
}

#[wasm_bindgen(js_name = isLost)]
pub fn is_lost() -> bool {
    MINESWEEPER.with(|ms| ms.borrow().is_lost())
}

#[wasm_bindgen(js_name = firstOpen)]
pub fn first_open() -> bool {
    MINESWEEPER.with(|ms| ms.borrow().is_first_opening())
}
