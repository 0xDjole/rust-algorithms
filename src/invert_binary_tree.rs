use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let l2 = node.borrow().left.clone();
            let r2 = node.borrow().right.clone();
            node.borrow_mut().left = Self::invert_tree(r2);
            node.borrow_mut().right = Self::invert_tree(l2);
            return Some(node);
        }
        None
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invert_binary_tree() {
        let mut node1 = TreeNode::new(10);
        let mut node2 = TreeNode::new(5);
        let node3 = TreeNode::new(15);
        let node4 = TreeNode::new(2);

        node2.left = Some(Rc::new(RefCell::new(node4)));
        node1.left = Some(Rc::new(RefCell::new(node2)));
        node1.right = Some(Rc::new(RefCell::new(node3)));

        let result = Solution::invert_tree(Some(Rc::new(RefCell::new(node1))));
        println!("RESULT IS {:?} ", result);
    }
}
