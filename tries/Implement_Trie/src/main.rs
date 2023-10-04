use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    is_end: bool,
    node: HashMap<u8, TrieNode>
}

struct Trie {
    root: TrieNode
}
/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    fn new() -> Self {
        Self {
            root: Default::default()
        }
    }
    
    fn insert(&mut self, word: String) {
        let mut curr_node = &mut self.root;
        for c in word.chars() {
            let next_node = curr_node.node.entry(c as u8 - 'a' as u8).or_default();
            curr_node = next_node;
        }
        curr_node.is_end = true;
    }

    fn __search__(&self, word: String, is_prefix: bool) -> bool {
        let mut curr_node = &self.root;
        for c in word.chars() {
            match curr_node.node.get(&(c as u8 - 'a' as u8)) {
                Some(node) => curr_node = node,
                None => return false,
            }
        }
        is_prefix || curr_node.is_end
    }
    
    fn search(&self, word: String) -> bool {
        self.__search__(word, false)
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        self.__search__(prefix, true)
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

fn main() {
    println!("Hello, world!");
}
