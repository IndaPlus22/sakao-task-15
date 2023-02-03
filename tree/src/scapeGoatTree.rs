#[derive(Debug, Clone)]
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

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left
            && self.right == other.right
            && self.parent == other.parent
            && self.value == other.value
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
        // if the tree is empty, insert the value at the root
        if self.size == 0 {
            self.root = Node::new(val);
            self.size += 1;
            return;
        }
        let mut currentNode: Node = self.root.clone();
        let mut potentialParent: Node = Node::new(0);

        // find the correct position for the node
        while currentNode.value != 0 {
            potentialParent = currentNode.clone();
            if val < currentNode.value {
                currentNode = *currentNode.left;
            } else {
                currentNode = *currentNode.right;
            }
        }

        let mut node = Node::new(val);
        if val < potentialParent.value {
            potentialParent.left = Box::new(node.clone());
        } else {
            potentialParent.right = Box::new(node.clone());
        }
        node.left = Box::new(Node::new(0));
        node.right = Box::new(Node::new(0));
        node.parent = Box::new(potentialParent);

        self.size += 1;

        let scapegoat = self.findScapegoat(&node);
        if scapegoat.is_some() {
            self.rebalance(scapegoat.unwrap());
        }
    }

    pub fn findScapegoat(&self, node: &Node) -> Option<Node> {
        if node == &self.root {
            return None;
        }
        let mut _node = node;
        while self.isBalancedAtNode(_node) {
            if _node == &self.root {
                return None;
            }
            _node = &_node.parent;
        }

        Some(_node.clone())
    }

    fn isBalancedAtNode(&self, node: &Node) -> bool {
        if (self.sizeOfSubtree(&node.left) - self.sizeOfSubtree(&node.right)).abs() <= 1 {
            true
        } else {
            false
        }
    }

    fn sizeOfSubtree(&self, node: &Node) -> i64 {
        if node.value == 0 {
            return 0;
        }
        1 + self.sizeOfSubtree(&node.left) + self.sizeOfSubtree(&node.right)
    }

    fn rebalance(&mut self, scapegoat: Node) {
        // TODO
    }

    fn flatten(&self, node: &Node, nodes: Vec<Node>) -> Vec<Node> {
        let mut nodes = Vec::new();
        if node.value == 0 {
            return nodes;
        }
        self.flatten(&node.left, nodes.clone());
        nodes.push(node.clone());
        self.flatten(&node.right, nodes.clone());

        nodes
    }

    fn buildTreeFromSortedNodes(&mut self, nodes: Vec<Node>, start: usize, end: usize) -> Node {
        // TODO
    }

    pub fn find(&self, val: u64) -> bool {
        let mut currentNode: Node = self.root.clone();
        while currentNode.value != 0 {
            if currentNode.value == val {
                return true;
            } else if val < currentNode.value {
                currentNode = *currentNode.left;
            } else {
                currentNode = *currentNode.right;
            }
        }
        false
    }
}
