// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
//
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub struct Solution {}
impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let l1_len = get_len(&l1);
        let l2_len = get_len(&l2);

        match l2_len > l1_len {
            true => {
                adjust_linked_list(&mut l1, &mut l2);
                l2
            }
            false => {
                adjust_linked_list(&mut l2, &mut l1);
                l1
            }
        }
    }
}

fn adjust_linked_list(
    mut shorter_list: &mut Option<Box<ListNode>>,
    mut longer_eq_list: &mut Option<Box<ListNode>>,
) {
    while shorter_list.is_some() && longer_eq_list.is_some() {
        let number = shorter_list.as_ref().unwrap().val + longer_eq_list.as_ref().unwrap().val;
        match number < 10 {
            true => longer_eq_list.as_mut().unwrap().val = number,
            false => {
                longer_eq_list.as_mut().unwrap().val = number - 10;
                match longer_eq_list.as_ref().unwrap().next.is_some() {
                    true => longer_eq_list.as_mut().unwrap().next.as_mut().unwrap().val += 1,
                    false => {
                        longer_eq_list.as_mut().unwrap().next = Some(Box::new(ListNode::new(1)))
                    }
                }
            }
        }
        shorter_list = &mut shorter_list.as_mut().unwrap().next;
        longer_eq_list = &mut longer_eq_list.as_mut().unwrap().next;
    }
    while longer_eq_list.is_some() && longer_eq_list.as_ref().unwrap().val == 10 {
        longer_eq_list.as_mut().unwrap().val = 0;
        match longer_eq_list.as_ref().unwrap().next.is_some() {
            true => longer_eq_list.as_mut().unwrap().next.as_mut().unwrap().val += 1,
            false => {
                longer_eq_list.as_mut().unwrap().next = Some(Box::new(ListNode::new(1)))
            }
        }
        longer_eq_list = &mut longer_eq_list.as_mut().unwrap().next;
    }
}
fn get_len(mut curr: &Option<Box<ListNode>>) -> i32 {
    let mut count = 0;
    while let Some(node) = curr {
        count += 1;
        curr = &node.next;
    }
    count
}

fn main() {
    println!("Hello, world!");
}
