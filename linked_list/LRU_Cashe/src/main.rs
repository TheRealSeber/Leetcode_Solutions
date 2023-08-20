use std::collections::{HashMap, VecDeque};

struct LRUCache {
    hash: HashMap<i32, i32>,
    stack: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            hash: HashMap::with_capacity(capacity as usize),
            stack: VecDeque::with_capacity(capacity as usize),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.hash.get(&key) {
            Some(v) => {
                self.stack.retain(|v| v != &key);
                self.stack.push_back(key);
                *v
            },
            _ => -1,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        match self.hash.get_mut(&key) {
            Some(v) => {
                *v = value;
                self.stack.retain(|v| v != &key);
                self.stack.push_back(key);
            },
            None => {
                match self.stack.len() == self.stack.capacity() {
                    true => {
                        let key_to_remove = self.stack.pop_front().unwrap();
                        self.hash.remove(&key_to_remove);
                        self.hash.insert(key, value);
                        self.stack.push_back(key)
                    },
                    false => {
                        self.hash.insert(key, value);
                        self.stack.push_back(key);
                    },
                }
                
            },
        }
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

fn main() {
    let mut obj = LRUCache::new(2);
    obj.put(1, 1);
    obj.put(2,2);
    let ret_1: i32 = obj.get(1);
    obj.put(3, 3);
    let ret_2: i32 = obj.get(2);
    let ret_3: i32 = obj.get(3);
    
    println!("Hello, world!, {}, {}, {}", ret_1, ret_2, ret_3);
}
