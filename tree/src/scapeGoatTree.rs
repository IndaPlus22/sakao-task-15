

struct Node {
    left: Box<Node>,
    right: Box<Node>,
    parent: Box<Node>,
    value: u64,
}

impl Node {
    fn new(val: u64) -> Node {
        Node {
            left: Box::new(Node::new(0)),
            right: Box::new(Node::new(0)),
            parent: Box::new(Node::new(0)),
            value: val,
        }
    }
}

pub struct ScapeGoatTree {
    root: Node,
    size: u64,
}

impl ScapeGoatTree {
    pub fn new() -> ScapeGoatTree {
        ScapeGoatTree {
            root: Node::new(0),
            size: 0,
        }
    }

    pub fn insert(&mut self, val: u64) {
        // self.size += 1;
        // self.root = Node::new(val);
    }

    pub fn rebalance(&mut self) {
        // TODO
    }
}