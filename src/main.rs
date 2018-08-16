// Copyright © 2018 Bart Massey
// This program is licensed under the "MIT License". Please
// see the file `LICENSE` in this distribution for license
// terms.

//! Chomp player in Rust.

extern crate rand;

use rand::Rng;

use std::collections::HashSet;
use std::fmt::{self, Display};
use std::io::Write;

/// Structure representing a candy bar to be chomped.
#[derive(Clone, Debug)]
struct Bar(HashSet<(u64, u64)>);
    
impl Bar {
    /// Create a new `Bar` with the given `width` and `height`.
    fn new(width: u64, height: u64) -> Bar {
        let mut bar = HashSet::with_capacity((width * height) as usize);
        for row in 0..height {
            for col in 0..width {
                bar.insert((row, col));
            }
        }
        Bar(bar)
    }

    /// True iff the given `row` / `col` is occupied.
    fn contains(&self, row: u64, col: u64) -> bool {
        self.0.contains(&(row, col))
    }

    /// Chomp at given position in a `Bar`, removing the
    /// given square and everything below and to the right
    /// of it.
    fn chomp(&mut self, row0: u64, col0: u64) {
        assert!(self.contains(row0, col0));
        for row in row0.. {
            if !self.contains(row, 0) {
                return;
            }
            for col in col0.. {
                if !self.0.remove(&(row, col)) {
                    break;
                }
            }
        }
    }
}

#[test]
fn new_bar_ok() {
    let b = Bar::new(4, 5);
    assert!(b.contains(3, 2));
    assert!(b.contains(4, 3));
    assert!(!b.contains(5, 4));
    assert!(b.0.len() == 20);
}

#[test]
fn chomp_works() {
    let mut b = Bar::new(4, 5);
    assert!(b.contains(4, 3));
    b.chomp(2, 2);
    assert!(!b.contains(2, 2));
    assert!(b.contains(1, 3));
    assert!(b.contains(3, 1));
}

impl Display for Bar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0.. {
            if !self.contains(row, 0) {
                break;
            }
            for col in 0.. {
                if self.contains(row, col) {
                    if row == 0 && col == 0 {
                        write!(f, "x")?;
                    } else {
                        write!(f, "o")?;
                    }
                } else {
                    break;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn get_move(b: &Bar) -> (u64, u64) {
    loop {
        println!("{}", b);
        print!("move: ");
        std::io::stdout().flush().unwrap();
        let mut moove = String::new();
        std::io::stdin().read_line(&mut moove).unwrap();
        let fields: Vec<&str> = moove.split_whitespace().collect();
        if fields.len() != 2 {
            println!("bad move format");
            continue;
        }
        let row: u64 = match fields[0].parse() {
            Err(e) => { println!("{}", e); continue; },
            Ok(v) => v,
        };
        let col: u64 = match fields[1].parse() {
            Err(e) => { println!("{}", e); continue; },
            Ok(v) => v,
        };
        if !b.contains(row, col) {
            println!("illegal move");
            continue;
        }
        return (row, col);
    }
}


impl Bar {
    /// Return true iff the game is a win for the side on move.
    fn negamax(&self) -> Option<(u64, u64)> {
        // Base case: there is just poison.
        if self.0.len() == 1 {
            return None;
        }

        // Recursive case: try to find a way to make
        // opponent eat poison.
        for &(row, col) in self.0.iter() {
            if row == 0 && col == 0 {
                continue;
            }
            let mut next = self.clone();
            next.chomp(row, col);
            if next.negamax().is_none() {
                return Some((row, col));
            }
        }
        None
    }

    /// Get a random move. May be poison.
    fn random_move(&self) -> (u64, u64) {
        let mut rng = rand::thread_rng();
        let moves: Vec<(u64, u64)> = self.0.iter().cloned().collect();
        return *rng.choose(&moves).unwrap();
    }
}

fn main() {
    let mut b = Bar::new(4, 3);
    loop {
        println!();
        println!("{}", b);
        let moove = match b.negamax() {
            Some(m) => m,
            None => b.random_move(),
        };
        if moove == (0, 0) {
            println!("poisoned: game over");
            return;
        }
        let (row, col) = moove;
        println!("computer move: {} {}", row, col);
        b.chomp(row, col);

        let moove = get_move(&b);
        if moove == (0, 0) {
            println!("poisoned: game over");
            return;
        }
        let (row, col) = moove;
        b.chomp(row, col);
    }
}
