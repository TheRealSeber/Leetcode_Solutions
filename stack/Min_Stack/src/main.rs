use std::boxed::Box;

struct Node {
    number: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Self {
            number: val,
            next: None,
        }
    }
}

struct MinStack {
    top: Option<Node>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self { top: None }
    }

    fn push(&mut self, val: i32) {
        let mut node = Node::new(val);
        if let Some(top) = std::mem::replace(&mut self.top, None) {
            node.next = Some(Box::new(top));
        }
        self.top = Some(node);
    }

    fn pop(&mut self) {
        if let Some(top) = std::mem::replace(&mut self.top, None) {
            self.top = match top.next {
                Some(node) => Some(*node),
                None => None,
            };
        }
    }

    fn top(&mut self) -> Option<i32> {
        if let Some(top) = std::mem::replace(&mut self.top, None) {
            self.top = match top.next {
                Some(node) => Some(*node),
                None => None,
            };
            return Some(top.number);
        }
        None
    }

    fn get_min(&self) -> Option<i32> {
        let mut min = std::i32::MAX;
        let mut current = self.top.as_ref();
        while let Some(node) = current {
            if node.number < min {
                min = node.number;
            }
            current = node.next.as_ref().map(|boxed_node| &**boxed_node);
        }
        if min == std::i32::MAX {
            return None;
        }
        Some(min)
    }        
}

fn main() {
    let mut stack = MinStack::new();
    stack.push(5);
    stack.push(2);
    stack.push(1);
    stack.push(11);
    println!("{}", stack.get_min().unwrap());
    println!("{}", stack.top().unwrap());
    stack.pop();
    println!("{}", stack.top().unwrap());
    println!("{}", stack.get_min().unwrap());
}