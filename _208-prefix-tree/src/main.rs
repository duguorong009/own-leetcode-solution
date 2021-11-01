use std::collections::HashMap;

fn main() {
    let mut trie = Trie::new();
    trie.insert("apple".to_string());
    assert_eq!(trie.search("apple".to_string()), true);
    assert_eq!(trie.search("app".to_string()), false);
    assert_eq!(trie.starts_with("app".to_string()), true);
    trie.insert("app".to_string());
    assert_eq!(trie.search("app".to_string()), true);
    println!("{:?}", trie);
}

#[derive(Debug)]
struct Trie {
    children: HashMap<char, Trie>,
    end: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Trie {
            children: HashMap::new(),
            end: false,
        }
    }

    fn insert(&mut self, word: String) {
        let mut link = self;
        for c in word.chars() {
            link = link.children.entry(c).or_insert(Trie::new());
        }
        link.end = true;
    }

    fn search(&self, word: String) -> bool {
        let mut link = self;
        for c in word.chars() {
            if let Some(child) = link.children.get(&c) {
                link = child;
            } else {
                return false;
            }
        }
        link.end
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut link = self;
        for c in prefix.chars() {
            if let Some(child) = link.children.get(&c) {
                link = child;
            } else {
                return false;
            }
        }
        true
    }
}

/***
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
