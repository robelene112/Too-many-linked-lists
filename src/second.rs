#![allow(unused_variables, dead_code)]
#[derive(Debug)]
pub struct List {
    head_node: Pointer,
}

type Pointer = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    value: i32,
    next: Pointer,
}

impl List {
    pub fn new() -> Self {
        List { head_node: None }
    }

    pub fn push(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: self.head_node.take(),
        });
        self.head_node = Some(new_node)
    }

    pub fn pop(&mut self) -> Option<i32> {
        match self.head_node.take() {
            None => None,

            Some(node) => {
                self.head_node = node.next;
                Some(node.value)
            }
        }
    }

    pub fn vecify_list(&mut self) -> Vec<i32> {
        let mut result = vec![];
        let mut headnode = self.head_node.take();
        while let Pointer::Some(mut boxed_node) = headnode {
            result.push(boxed_node.value);
            headnode = boxed_node.next.take();
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
    println!("{:#?}, \n{:#?}", list, value);

    println!("{:?}", list.vecify_list());
}
