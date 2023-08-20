// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
pub struct Solution {}
impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        if head.as_ref().unwrap().next.is_none() {
            head = None;
            return head;
        }
        let mut count = 0;
        let mut curr = &head;
        while let Some(node) = curr {
            count += 1;
            curr = &node.next;
        }

        if count == n {
            return head.unwrap().next;
        }
        
        let mut curr = &mut head;
        for _ in 1..count - n  {
            curr = &mut curr.as_mut().unwrap().next;
        }

        if curr.as_ref().unwrap().next.as_ref().unwrap().next.is_some() {
            curr.as_mut().unwrap().next = curr.as_mut().unwrap().next.as_mut().unwrap().next.take();
        } else {
            curr.as_mut().unwrap().next.take();
        }

        head
    }
}

fn main() {
    println!("Hello, world!");
}
