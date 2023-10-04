use std::collections::HashMap;

#[derive(Default)]
struct WordDictionary {
    is_end: bool,
    childrens: HashMap<u8, WordDictionary>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        Self::default()
    }

    fn add_word(&mut self, word: String) {
        let mut trie = self;

        for b in word.bytes() {
            trie = trie.childrens.entry(b - b'a').or_default()
        }
        trie.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        Self::search_trie(&self, word.as_bytes())
    }
    fn search_trie(&self, word: &[u8]) -> bool {
        if let Some(&c) = word.first() {
            if c == b'.' {
                for letter in self.childrens.values() {
                    if Self::search_trie(&letter, &word[1..]) {
                        return true;
                    }
                }
            } else if let Some(node) = self.childrens.get(&(c - b'a')) {
                Self::search_trie(&node, &word[1..]);
            }
            return false;
        } else {
            self.is_end
        }
    }
}
/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */

fn main() {
    println!("Hello, world!");
}
