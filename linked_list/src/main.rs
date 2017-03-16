extern crate core;

use core::default::Default;

struct Node {
    data: u32,
    next: Option<Box<Node>>
}

impl Node {

    pub fn new(data: u32) -> Self {
        Node {
            data: data,
            next: None
        }
    }
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {

    pub fn new() -> Self {
        LinkedList {
            head: None,
        }
    }

    pub fn push(&mut self, data: u32) {

        let ref mut current = self.head;

        loop {

            current = match *current {
                None => {
                    &mut Some(Box::new(Node::new(data)))
                }
                Some(ref cur) => {
                    &mut cur.next
                }
            }
        }
    }
}

fn main() {

    let mut list = LinkedList::new();

    list.push(10);
    list.push(20);
}
