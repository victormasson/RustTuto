use crate::random::random_range;
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

#[derive(Debug, Clone)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_fields: HashSet<Position>,
    lost: bool,
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);

                if !self.open_fields.contains(&pos) {
                    if self.lost && self.mines.contains(&pos) {
                        f.write_str("💣 ")?;
                    } else if self.flagged_fields.contains(&pos) {
                        f.write_str("🚩 ")?;
                    } else {
                        f.write_str("🟦 ")?;
                    }
                } else if self.mines.contains(&pos) {
                    f.write_str("💣 ")?;
                } else {
                    let mine_count = self.neighboring_mines(pos);

                    if mine_count > 0 {
                        write!(f, " {} ", mine_count)?;
                    } else {
                        f.write_str("⬜ ")?;
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
            mines: {
                let mut mines = HashSet::new();

                while mines.len() < mine_count {
                    mines.insert((random_range(0, width), random_range(0, height)));
                }
                mines
            },
            flagged_fields: HashSet::new(),
            lost: false,
        }
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

    #[cfg(test)]
    pub fn open_test(&mut self, pos: Position) -> Option<OpenResult> {
        if self.flagged_fields.contains(&pos) {
            return None;
        }

        self.open_fields.insert(pos);

        match self.mines.contains(&pos) {
            true => Some(OpenResult::Mine),
            false => Some(OpenResult::NoMine(0)),
        }
    }

    pub fn open(&mut self, pos: Position) -> Option<OpenResult> {
        if self.lost || self.open_fields.contains(&pos) || self.flagged_fields.contains(&pos) {
            return None;
        }

        self.open_fields.insert(pos);

        match self.mines.contains(&pos) {
            true => {
                self.lost = true;
                Some(OpenResult::Mine)
            }
            false => {
                let mine_count = self.neighboring_mines(pos);

                if mine_count == 0 {
                    for neighbor in self.iter_neighbors(pos) {
                        self.open(neighbor);
                    }
                }

                Some(OpenResult::NoMine(0))
            }
        }
    }

    pub fn toggle_flag(&mut self, pos: Position) {
        if self.lost || self.open_fields.contains(&pos) {
            return;
        }

        if self.flagged_fields.contains(&pos) {
            self.flagged_fields.remove(&pos);
        } else {
            self.flagged_fields.insert(pos);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::minesweeper::{Minesweeper, Position};

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
                minesweeper_copy.open_test(pos);
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
        let res = minesweeper.open_test((6, 6));

        assert!(res.is_none());
        println!("{}", minesweeper);
        println!("{:#?}", res);
    }
}
