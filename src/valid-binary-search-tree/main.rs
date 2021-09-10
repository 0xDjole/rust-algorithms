struct BST<T> {
    value: T,
    left: Leaf<T>,
    right: Leaf<T>,
}

type Leaf<T> = Option<Box<BST<T>>>;

impl<T> BST<T>
where
    T: Ord,
{
    pub fn new(value: T, left: Leaf<T>, right: Leaf<T>) -> BST<T> {
        BST { value, left, right }
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

        if let Some(right_leaf) = &self.right {
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
    let mut node1 = BST::new(10, None, None);
    let mut node2 = BST::new(5, None, None);
    let node3 = BST::new(15, None, None);
    let node4 = BST::new(2, None, None);

    let node4_leaf = Some(Box::new(node4));
    node2.left = node4_leaf;
    let node2_leaf = Some(Box::new(node2));
    let node3_leaf = Some(Box::new(node3));

    node1.left = node2_leaf;
    node1.right = node3_leaf;

    let is_valid = node1.is_valid();
    println!("IS VALID {:?} ", is_valid);
}
