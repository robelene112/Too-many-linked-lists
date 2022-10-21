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
    pub fn new() -> Self {
        List {
            head_node: Pointer::Empty,
        }
    }

    pub fn push(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: mem::replace(&mut self.head_node, Pointer::Empty),
        });
        self.head_node = Pointer::Elem(new_node)
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head_node, Pointer::Empty) {
            Pointer::Empty => None,

            Pointer::Elem(node) => {
                self.head_node = node.next;
                Some(node.value)
            }
        }
    }

    pub fn vecify_list(&mut self) -> Vec<i32> {
        let mut result = vec![];
        let mut headnode = mem::replace(&mut self.head_node, Pointer::Empty);
        while let Pointer::Elem(mut boxed_node) = headnode {
            result.push(boxed_node.value);
            headnode = mem::replace(&mut boxed_node.next, Pointer::Empty)
        }

        result
    }
}

fn main() {
    let mut list: List = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    let value = list.pop();
    println!("{:#?}, {:#?}", list, value);

    println!("{:?}", list.vecify_list());
}
