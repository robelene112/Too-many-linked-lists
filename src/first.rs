#![allow(unused_variables, dead_code)]
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
        let new_node = Node {
            value,
            next: Pointer::Empty,
        };
        self.head_node = Pointer::Elem(Box::new(new_node));
    }
}

fn main() {
    let mut list: List<i32> = List::new();
    list.push(1);
    println!("{:#?}", list)
}
