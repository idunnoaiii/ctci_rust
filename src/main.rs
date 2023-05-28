use std::{sync::{Arc, Mutex}, thread};

mod ds;
    
// Make it wor
use std::mem::size_of_val;

fn main() {
} 

fn first_world(s: &String) -> usize {
    let chars = s.as_bytes();
    
    for (idx, &item) in chars.iter().enumerate() {
        if item == b' ' {
            return idx;
        }
    }

    return s.len();
}



