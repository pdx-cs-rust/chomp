// Copyright © 2018 Bart Massey

//! Chomp player in Rust.

use std::collections::HashSet;
use std::fmt::{self, Display};

/// Structure representing a candy bar to be chomped.
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

fn main() {
    let b = Bar::new(4, 3);
    print!("{}", b);
}
