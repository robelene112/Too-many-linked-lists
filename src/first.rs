#![allow(unused_variables, dead_code)]
use std::mem;
#[derive(Debug)]
pub struct List<T> {
    head_node: Pointer<T>,
}
#[derive(Debug)]
enum Pointer<T> {
    Empty,
    Elem(Box<Node<T>>),
}
#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Pointer<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            head_node: Pointer::Empty,
        }
    }

    fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: mem::replace(&mut self.head_node, Pointer::Empty),
        });
        self.head_node = Pointer::Elem(new_node)
    }
}

fn main() {
    let mut list: List<String> = List::new();
    list.push("Hello".to_owned());
    list.push(", ".to_owned());
    list.push("world".to_owned());
    list.push("!".to_owned());
    println!("{:#?}", list)
}
