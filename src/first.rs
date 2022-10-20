#![allow(unused_variables, dead_code)]
use std::mem;
#[derive(Debug)]
pub struct List {
    head_node: Pointer,
}
#[derive(Debug)]
enum Pointer {
    Empty,
    Elem(Box<Node>),
}
#[derive(Debug)]
struct Node {
    value: i32,
    next: Pointer,
}

impl List {
    fn new() -> Self {
        List {
            head_node: Pointer::Empty,
        }
    }

    fn push(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: mem::replace(&mut self.head_node, Pointer::Empty),
        });
        self.head_node = Pointer::Elem(new_node)
    }

    pub fn pop(&mut self) -> Option<i32> {
        let value;
        match mem::replace(&mut self.head_node, Pointer::Empty) {
            Pointer::Empty => {
                value = None;
            }

            Pointer::Elem(node) => {
                value = Some(node.value);
                self.head_node = node.next;
            }
        };

        value
    }
}

fn main() {
    let mut list: List = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    let value = list.pop();
    println!("{:#?}, {:#?}", list, value)
}
