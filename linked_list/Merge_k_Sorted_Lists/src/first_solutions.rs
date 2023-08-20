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
        let mut res: Option<Box<ListNode>> = None;
        let mut curr = &mut res;
        loop {
            let mut lowest: Option<(i32, i32)> = None;
            lists.iter_mut().enumerate().for_each(|(idx, v)| {
                if let Some(node) = v {
                    match_lowest(&mut lowest, node, idx);
                }
            });
            match lowest {
                Some((idx, val)) => {
                    let head = lists.get_mut(idx as usize).unwrap().take();
                    lists[idx as usize] = head.unwrap().next;
                    let new_node = Some(Box::new(ListNode::new(val)));
                    match curr {
                        Some(node) => {
                            node.next = new_node;
                            curr = &mut node.next;
                        },
                        None => *curr = new_node,
                    }
                },
                None => break,
            }
        }
        res
    }
}

fn match_lowest(lowest: &mut Option<(i32, i32)>, node: &mut Box<ListNode>, idx: usize) {
    match lowest {
        Some(num) => {
            if node.val < num.1 {
                *lowest = Some((idx as i32, node.val));
            }
        }
        None => *lowest = Some((idx as i32, node.val)),
    }
}