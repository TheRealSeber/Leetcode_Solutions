// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
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

pub struct Solution {}
impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut res = ListNode::new(i32::MIN);
        let mut curr = &mut res;
        let mut remain = lists.len();

        if remain == 0 {
            return res.next
        }

        loop {
            lists.sort_unstable_by(|a, b| {
                if a.is_none() {
                    return std::cmp::Ordering::Greater;
                } else if b.is_none() {
                    return std::cmp::Ordering::Less;
                } else {
                    return a.as_ref().unwrap().val.cmp(&b.as_ref().unwrap().val);
                }
            });

            if lists[0].is_none() {
                break;
            }

            let lowest_val = lists[0].as_ref().unwrap().val;

            let mut cleared = 0;

            for i in 0..remain {
                let mut node = lists[i].take();

                while node.is_some() && node.as_ref().unwrap().val == lowest_val {
                    curr.next = node;
                    node = curr.next.as_mut().unwrap().next.take();
                    curr = curr.next.as_mut().unwrap();
                }

                if node.is_none() {
                    cleared += 1;
                }

                lists[i] = node;
            }

            remain -= cleared;
        }
        res.next
    }
}

fn main() {
    println!("Hello, world!");
}
