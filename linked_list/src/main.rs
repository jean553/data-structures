extern crate core;

use core::default::Default;

struct Node<'a> {
    data: u64,
    next: Option<&'a Node<'a>>
}

#[derive(Default)]
struct LinkedList<'a> {
    head: Option<Node<'a>>
}

impl<'a> LinkedList<'a> {
    
    fn insert_at_begin(&mut self, data: u64) {

        let node = Node{
            data: data,
            next: None
        };
        
        self.head = Some(node);
    }
}

fn main() {

    let mut list = LinkedList{..Default::default()};

    list.insert_at_begin(10);
}
