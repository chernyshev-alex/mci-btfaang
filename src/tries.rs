use std::{collections::HashMap};

struct TrieNode {
    is_end: bool,
    keys: HashMap<char, TrieNode>,
}

impl TrieNode {
    pub fn new() -> Self {
        Self {
            is_end: false,
            keys: HashMap::new(),
        }
    }
}
struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        let mut it = word.chars();
        while let Some(ch) = it.next() {
            if !node.keys.contains_key(&ch) {
                node.keys.insert(ch, TrieNode::new());
            }
            node = node.keys.get_mut(&ch).unwrap();
        }
        node.is_end = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut node = &self.root;
        let mut it = word.chars();
        let mut result = false;
        while let Some(ch) = it.next() {
            let v = node.keys.get(&ch);
            match v {
                Some(n) => result = n.is_end,
                None => return false,
            }
            node = v.unwrap();
        }
        result
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;
        let mut it = prefix.chars();
        while let Some(ch) = it.next() {
            let v = node.keys.get(&ch);
            if v.is_none() {
                return false;
            }
            node = v.unwrap();
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // https://leetcode.com/problems/implement-trie-prefix-tree/
    #[test]
    fn trie_test() {
        let word = String::from("apple");
        let mut trie = Trie::new();

        assert_eq!(false, trie.search("".to_string()));

        trie.insert("apple".to_string());
        assert_eq!(true, trie.search("apple".to_string()));
        assert_eq!(false, trie.search("app".to_string()));
        assert_eq!(true, trie.starts_with("apple".to_string()));

        trie.insert("dog".to_string());
        trie.insert("app".to_string());
        assert_eq!(true, trie.search("app".to_string()));
    }
    #[test]
    fn trie_test2() {
        let mut trie = Trie::new();
        trie.insert("hello".to_string());

        // assert_eq!(false, trie.search("hell".to_string()));
        assert_eq!(false, trie.search("helloa".to_string()));
        // assert_eq!(true, trie.search("hello".to_string()));
        
        // assert_eq!(true, trie.starts_with("hell".to_string()));
        // assert_eq!(false, trie.starts_with("helloa".to_string()));
        // assert_eq!(true, trie.starts_with("hello".to_string()));
    }
}
