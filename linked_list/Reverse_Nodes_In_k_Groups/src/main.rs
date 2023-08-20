// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub struct Solution {}
impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let groups = count_nodes(&head)/k;
        let mut res = Box::new(ListNode::new(-1));
        let mut res_ref = &mut res;
        for _ in 0..groups {
            for _ in 0..k {
                let mut current = head.take();
                head = current.as_mut().unwrap().next.take();
                current.as_mut().unwrap().next = res_ref.next.take();
                res_ref.next = current;
            }

            while let Some(ref mut next) = res_ref.next {
                res_ref = next;
            }
        }
        res_ref.next = head;
        res.next
    }
}

fn count_nodes(mut curr: &Option<Box<ListNode>>) -> i32 {
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
