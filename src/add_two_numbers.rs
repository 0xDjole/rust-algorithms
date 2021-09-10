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
        let mut l1_current = l1;
        let mut l2_current = l2;
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut current = head.as_mut();
        let mut carry = 0;

        while l1_current.is_some() || l2_current.is_some() || carry != 0 {
            let mut sum = carry;

            if let Some(node) = l1_current {
                sum += node.val;
                l1_current = node.next;
            }

            if let Some(node) = l2_current {
                sum += node.val;
                l2_current = node.next;
            }

            carry = sum / 10;

            if let Some(node) = current {
                node.next = Some(Box::new(ListNode::new(sum % 10)));
                current = node.next.as_mut();
            }
        }

        if carry > 0 {
            current.unwrap().next = Some(Box::new(ListNode::new(carry)))
        }

        head.unwrap().next
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_two_numbers() {
        let mut node1 = ListNode::new(3);
        node1.next = Some(Box::new(ListNode::new(6)));
        node1.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(5)));

        let mut node2 = ListNode::new(2);
        node2.next = Some(Box::new(ListNode::new(4)));
        node2.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(8)));

        let result = Solution::add_two_numbers(Some(Box::new(node1)), Some(Box::new(node2)));
        println!("RESULT {:?}", result);
    }
}
