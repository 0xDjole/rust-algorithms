#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
    }
}

fn main() {
    let node1 = ListNode::new(2);
    node1.next = Some(Box::new(ListNode::new(4)));
    node1.next = Some(Box::new(ListNode::new(3)));

    let node2 = ListNode::new(5);
    node2.next = Some(Box::new(ListNode::new(6)));
    node2.next = Some(Box::new(ListNode::new(4)));
}
