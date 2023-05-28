use std::fmt::Debug;

use std::mem;

#[derive(Debug)]
pub struct List<T: std::fmt::Debug> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<*mut Node<T>>;

#[derive(Debug)]
struct Node<T: std::fmt::Debug> {
    value: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T: std::fmt::Debug> Node<T> {
    pub fn new(elem: T) -> *mut Self {
        let node = Box::new(Node {
            value: elem,
            next: None,
            prev: None,
        });
        Box::into_raw(node)
    }
}
