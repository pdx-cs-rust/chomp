// Copyright © 2018 Bart Massey

//! Chomp player in Rust.

use std::collections::HashSet;

/// Structure representing a candy bar to be chomped.
struct Bar(HashSet<(u64, u64)>);
    
impl Bar {
    fn new(width: u64, height: u64) -> Bar {
        let mut bar = HashSet::with_capacity(width * height);
        for row in 0..height {
            for col in 0..width {
                bar.insert((row, col));
            }
        }
        Bar(bar)
    }
}


fn main() {
    
}
