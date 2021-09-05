struct Node {
    value: i32,
    left: Leaf,
    right: Leaf,
}

type Leaf = Option<Box<Node>>;

impl Node {
    pub fn new(value: i32, left: Leaf, right: Leaf) -> Node {
        Node { value, left, right }
    }

    pub fn is_valid(&self) -> bool {
        let mut valid = true;
        if let Some(left_leaf) = &self.left {
            if left_leaf.value < self.value && left_leaf.is_valid() {
                valid = true
            } else {
                valid = false
            }
        }

        if !valid {
            return false;
        }

        if let Some(right_leaf) = &self.left {
            if right_leaf.value < self.value && right_leaf.is_valid() {
                valid = true
            } else {
                valid = false
            }
        }

        valid
    }
}

fn main() {
    let mut node1 = Node::new(10, None, None);
    let node2 = Node::new(5, None, None);
    let node3 = Node::new(20, None, None);

    node1.left = Some(Box::new(node2));
    node1.right = Some(Box::new(node3));

    let is_valid = node1.is_valid();
    println!("IS VALID {:?} ", is_valid);
}
