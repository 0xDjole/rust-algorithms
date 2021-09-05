use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    value: i32,
    left: Leaf,
    right: Leaf,
}

type Leaf = Rc<RefCell<Option<Node>>>;

impl Node {
    pub fn new(value: i32, left: Leaf, right: Leaf) -> Node {
        Node { value, left, right }
    }

    pub fn is_valid(&self) -> bool {
        let mut valid = true;
        if let Some(left_leaf) = self.left.borrow().as_ref() {
            if left_leaf.value < self.value && left_leaf.is_valid() {
                valid = true
            } else {
                valid = false
            }
        }

        if !valid {
            return false;
        }

        if let Some(right_leaf) = self.right.borrow().as_ref() {
            if right_leaf.value > self.value && right_leaf.is_valid() {
                valid = true
            } else {
                valid = false
            }
        }

        valid
    }
}

fn main() {
    let mut node1 = Node::new(10, Rc::new(RefCell::new(None)), Rc::new(RefCell::new(None)));
    let node2 = Rc::new(RefCell::new(Some(Node::new(
        5,
        Rc::new(RefCell::new(None)),
        Rc::new(RefCell::new(None)),
    ))));

    let node3 = Rc::new(RefCell::new(Some(Node::new(
        15,
        Rc::new(RefCell::new(None)),
        Rc::new(RefCell::new(None)),
    ))));

    let node4 = Rc::new(RefCell::new(Some(Node::new(
        3,
        Rc::new(RefCell::new(None)),
        Rc::new(RefCell::new(None)),
    ))));

    node1.left = Rc::clone(&node2);
    node1.right = Rc::clone(&node3);
    node2.borrow_mut().as_mut().unwrap().left = Rc::clone(&node4);

    let is_valid = node1.is_valid();
    println!("IS VALID {:?} ", is_valid);
}
