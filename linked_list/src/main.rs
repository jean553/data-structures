extern crate core;

struct Node {
    data: u32,
    next: Option<Box<Node>>
}

impl Node {

    /// Constructs a new node (first node for a linked list)
    ///
    /// This method is used to create the first node of a linked list
    ///
    /// Example:
    ///
    /// ```
    /// let node = Node::new(10);
    /// ```
    pub fn new(data: u32) -> Self {
        Node {
            data: data,
            next: None
        }
    }

    /// Append a new node at the end of the list
    ///
    /// Example:
    ///
    /// ```
    /// node.push(20);
    /// ```
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
