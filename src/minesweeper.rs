use wasm_bindgen::prelude::*;

use crate::random::{calculate_mines, random_range};
use std::{
    collections::HashSet,
    fmt::{Display, Write},
};

pub type Position = (usize, usize);

#[derive(Debug)]
pub enum OpenResult {
    Mine,
    NoMine(u8),
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatusGame {
    FirstOpening = 1,
    Playing = 2,
    Lost = 3,
    Win = 4,
}

#[derive(Debug, Clone)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>,
    mine_count: usize,
    flagged_fields: HashSet<Position>,
    lost: bool,
    status: StatusGame,
    first_opening: bool,
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);

                if !self.open_fields.contains(&pos) {
                    if self.lost && self.mines.contains(&pos) {
                        f.write_str("x ")?; //💣
                    } else if self.flagged_fields.contains(&pos) {
                        f.write_str("> ")?; //🚩
                    } else {
                        f.write_str("O ")?; //🟦
                    }
                } else if self.mines.contains(&pos) {
                    f.write_str("x ")?;
                } else {
                    let mine_count = self.neighboring_mines(pos);

                    if mine_count > 0 {
                        write!(f, " {} ", mine_count)?;
                    } else {
                        f.write_str("C ")?; //⬜
                    }
                }
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        Minesweeper {
            width,
            height,
            open_fields: HashSet::new(),
            mines: Minesweeper::calculate_mines(width, height, mine_count),
            mine_count: mine_count,
            flagged_fields: HashSet::new(),
            lost: false,
            status: StatusGame::FirstOpening,
            first_opening: true,
        }
    }

    pub fn get_status(&self) -> StatusGame {
        self.status
    }

    pub fn insert_mines_new(&mut self, width: usize, height: usize, mine_count: usize) {
        self.mines = calculate_mines(width, height, mine_count);
    }

    #[cfg(test)]
    pub fn insert_mines_debug(&mut self, mines: HashSet<Position>) {
        self.status = StatusGame::Playing;
        self.mines = mines;
    }

    fn calculate_mines(width: usize, height: usize, mine_count: usize) -> HashSet<(usize, usize)> {
        calculate_mines(width, height, mine_count)
    }

    fn insert_mines(&self, pos: Position) -> HashSet<(usize, usize)> {
        let mut mines = HashSet::new();

        while mines.len() < self.mine_count {
            let p = (random_range(0, self.width), random_range(0, self.height));

            let (x, y) = pos;

            if !(x - 1..=x + 1)
                .flat_map(move |i| (y - 1..=y + 1).map(move |j| (i, j)))
                .any(move |(i, j)| (i, j) == p)
            {
                mines.insert(p);
            }
        }
        mines
    }

    fn is_there_only_mines_left(&self) -> bool {
        let width = self.width;
        let height = self.height;

        let all_map = (0..=width - 1)
            .flat_map(move |i| (0..=height - 1).map(move |j| (i, j)))
            .collect::<HashSet<Position>>();

        println!("{} all_map", all_map.len());
        println!("{} open_fields", self.open_fields.len());

        let all_map_without_opened = all_map
            .iter()
            .filter(|&x| !&self.open_fields.contains(x))
            .cloned()
            .collect::<HashSet<(usize, usize)>>();

        println!("{} all_map_without_opened", all_map_without_opened.len());

        let mut set = all_map_without_opened;

        for pos in &self.flagged_fields {
            set.insert(*pos);
        }

        self.mines == set
    }

    fn iter_neighbors(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height;

        (x.max(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
            .filter(move |&pos| pos != (x, y))
    }

    fn neighboring_mines(&self, pos: Position) -> u8 {
        self.iter_neighbors(pos)
            .filter(|pos| self.mines.contains(pos))
            .count() as u8
    }

    pub fn open(&mut self, pos: Position) -> Option<OpenResult> {
        if self.status == StatusGame::FirstOpening {
            if self.mines.contains(&pos) {
                self.mines = self.insert_mines(pos);
            }

            self.status = StatusGame::Playing;
        }

        if self.open_fields.contains(&pos) {
            let mine_count = self.neighboring_mines(pos);
            let flag_count = self
                .iter_neighbors(pos)
                .filter(|neighbor| self.flagged_fields.contains(neighbor))
                .count() as u8;
            if mine_count == flag_count {
                for neighbor in self.iter_neighbors(pos) {
                    if !self.flagged_fields.contains(&neighbor)
                        && !self.open_fields.contains(&neighbor)
                    {
                        self.open(neighbor);
                    }
                }
            }
            return None;
        }

        if self.status == StatusGame::Lost
            || self.status == StatusGame::Win
            || self.flagged_fields.contains(&pos)
        {
            return None;
        }

        self.open_fields.insert(pos);

        match self.mines.contains(&pos) {
            true => {
                self.status = StatusGame::Lost;
                Some(OpenResult::Mine)
            }
            false => {
                let mine_count = self.neighboring_mines(pos);

                if mine_count == 0 {
                    for neighbor in self.iter_neighbors(pos) {
                        if !self.flagged_fields.contains(&neighbor) {
                            self.open(neighbor);
                        }
                    }
                }

                if self.is_there_only_mines_left() {
                    self.status = StatusGame::Win;

                    return None;
                }

                Some(OpenResult::NoMine(0))
            }
        }
    }

    pub fn toggle_flag(&mut self, pos: Position) {
        if self.status == StatusGame::Lost
            || self.status == StatusGame::Win
            || self.open_fields.contains(&pos)
        {
            return;
        }

        if self.flagged_fields.contains(&pos) {
            self.flagged_fields.remove(&pos);
        } else {
            self.flagged_fields.insert(pos);

            if self.is_there_only_mines_left() {
                self.status = StatusGame::Win;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashSet,
        sync::{Arc, Mutex},
    };

    use crate::minesweeper::{Minesweeper, Position, StatusGame};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn open_minesweeper_test() {
        let width = 10;
        let height = 10;
        let mut minesweeper = Arc::new(Mutex::new(Minesweeper::new(10, 10, 10)));
        let minesweeper_copy = Arc::clone(&minesweeper);

        let a = (0..=width)
            .flat_map(move |i| (0..=height).map(move |j| (i, j)))
            .collect::<Vec<Position>>();

        (0..=width)
            .flat_map(move |i| (0..=height).map(move |j| (i, j)))
            .for_each(move |pos| {
                let mut minesweeper_copy = minesweeper_copy.lock().unwrap();
                minesweeper_copy.open(pos);
            });

        let minesweeper = minesweeper.lock().unwrap();
        let nb_mine = minesweeper.neighboring_mines((6, 6));

        println!("{}", minesweeper);
        println!("nb mine {}", nb_mine)
    }

    #[test]
    fn flag_and_try_to_open_minesweeper_test() {
        let mut minesweeper = Minesweeper::new(10, 10, 10);
        minesweeper.toggle_flag((6, 6));
        let res = minesweeper.open((6, 6));

        assert!(res.is_none());
        println!("{}", minesweeper);
        println!("{:#?}", res);
    }

    #[test]
    fn try_to_open_minesweeper_test() {
        let mut minesweeper = Minesweeper::new(10, 10, 60);

        let res = minesweeper.open((6, 6));

        println!("{}", minesweeper);
        println!("{:#?}", res);
    }

    #[test]
    fn minesweeper_win_test() {
        let mut minesweeper = Minesweeper::new(3, 3, 1);

        let to_open = vec![
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 1),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ];

        let mines: HashSet<Position> = (vec![(0, 0)]).into_iter().collect();

        minesweeper.insert_mines_debug(mines);

        for pos in to_open {
            minesweeper.open(pos);
        }

        let res = minesweeper.get_status();

        assert_eq!(StatusGame::Win, res);

        println!("{}", minesweeper);
        println!("{:#?}", res);
    }
}
