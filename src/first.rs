#![allow(unused_variables, dead_code)]
#[derive(Debug)]
enum List<T> {
    Empty,
    Elem(T, Box<List<T>>),
}

fn main() {
    let list: List<i32> = List::Elem(1, Box::new(List::Elem(2, Box::new(List::Empty))));
    println!("{:#?}", list)
}
