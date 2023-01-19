pub struct Solution {}

use crate::util::linked_list::{to_list, ListNode};

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let (mut l1_end, mut l2_end) = (false, false);
        let mut carry = 0;

        let mut result = Some(Box::new(ListNode::new(0)));
        let mut p = &mut result;
        loop {
            let l1_val = match l1 {
                None => {
                    l1_end = true;
                    0
                },
                Some(node) => {
                    l1 = node.next;
                    node.val
                }
            };
            let l2_val = match l2 {
                None => {
                    l2_end = true;
                    0
                }
                Some(node) => {
                    l2 = node.next;
                    node.val
                }
            };

            if l1_end && l2_end && carry == 0 {
                break result.unwrap().next;
            }
            
            let sum = l1_val + l2_val + carry;
            carry = sum / 10;

            let current_node = Some(Box::new(ListNode::new(sum % 10)));
            p.as_mut().unwrap().next = current_node;
            p = &mut p.as_mut().unwrap().next;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_2() {
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
            to_list(vec![7, 0, 8])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![9, 9, 9, 9]), to_list(vec![9, 9, 9, 9, 9, 9])),
            to_list(vec![8, 9, 9, 9, 0, 0, 1])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![0]), to_list(vec![0])),
            to_list(vec![0])
        )
    }
}