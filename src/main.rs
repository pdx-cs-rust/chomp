// Copyright © 2018 Bart Massey

//! Chomp player in Rust.

use std::collections::HashSet;

/// Structure representing a candy bar to be chomped.
struct Bar(HashSet<(u64, u64)>);
    
impl Bar {
    fn new(width: u64, height: u64) -> Bar {
        let mut bar = HashSet::with_capacity((width * height) as usize);
        for row in 0..height {
            for col in 0..width {
                bar.insert((row, col));
            }
        }
        Bar(bar)
    }

    fn contains(&self, row: u64, col: u64) -> bool {
        self.0.contains(&(row, col))
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

fn main() {
    
}
