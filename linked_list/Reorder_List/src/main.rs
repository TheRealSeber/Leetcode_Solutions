impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let count = get_len(head);

        let mut half_head = get_middle_point(count, head).as_mut().unwrap().next.take();

        reverse_half(&mut half_head);

        merge(head, &mut half_head);
    }
}

fn merge(mut head: &mut Option<Box<ListNode>>, reversed_second_half: &mut Option<Box<ListNode>>) {
    while head.is_some() && reversed_second_half.is_some() {
        let left_next = &mut head.as_mut().unwrap().next;
        std::mem::swap(left_next, reversed_second_half);
        std::mem::swap(&mut left_next.as_mut().unwrap().next, reversed_second_half);

        head = &mut left_next.as_mut().unwrap().next;
    }
}

fn reverse_half(half_head: &mut Option<Box<ListNode>>) {
    let mut reversed_second_half = None;
    let curr = &mut half_head.take();
    while curr.is_some() {
        std::mem::swap(&mut curr.as_mut().unwrap().next, &mut reversed_second_half);
        std::mem::swap(curr, &mut reversed_second_half);
    }
    std::mem::swap(half_head, &mut reversed_second_half);
}

fn get_middle_point(count: i32, head: &mut Option<Box<ListNode>>) -> &mut Option<Box<ListNode>> {
    let mut curr = head;
    for _ in 0..(count - 1) / 2  {
        curr = &mut curr.as_mut().unwrap().next;
    }
    curr
}

fn get_len(head: &mut Option<Box<ListNode>>) -> i32 {
    let mut count = 0;
    let mut pnt = &*head;
    while let Some(obj) = pnt {
        count += 1;
        pnt = &obj.next;
    }
    count
}