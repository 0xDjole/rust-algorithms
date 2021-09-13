#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
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
    fn reverse(node: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = node;
        let mut previous = None;

        while !current.is_none() {
            let temporary = current.clone().unwrap().next;
            current.as_mut().unwrap().next = previous;
            previous = current;
            current = temporary;
        }

        previous
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reverse_linked_list() {
        let mut node1 = ListNode::new(3);
        node1.next = Some(Box::new(ListNode::new(6)));
        node1.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(5)));

        let result = Solution::reverse(Some(Box::new(node1)));
        println!("RESULT {:?}", result);
    }
}
