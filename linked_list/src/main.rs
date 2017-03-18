extern crate core;

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

    pub fn push(self, data: u32) {

        let mut current = self.next;

        loop {
            current = match current {
                Some(curr) => {curr.next}
                None => {
                    Some(Box::new(Node::new(data)));
                    break;
                }
            }
        }
    }
}

fn main() {

    let node = Node::new(10);
    node.push(20);
}
