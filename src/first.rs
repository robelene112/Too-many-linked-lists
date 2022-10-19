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

fn main() {
    let list = List {
        head_node: Pointer::Elem(Box::new(Node {
            value: 1,
            next: Pointer::Elem(Box::new(Node {
                value: 2,
                next: Pointer::Empty,
            })),
        })),
    };
    println!("{:#?}", list)
}
