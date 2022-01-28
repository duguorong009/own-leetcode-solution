use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Eq, Default, Debug)]
struct Trie {
    children: HashMap<char, Trie>,
    end: bool,
}

impl Trie {
    fn insert(&mut self, s: &str) {
        let mut link = self;
        for c in s.chars() {
            link = link.children.entry(c).or_default();
        }
        link.end = true;
    }

    fn search(&self, s: &str) -> bool {
        if s.is_empty() {
            return self.end;
        }
        let c = s.chars().next().unwrap();
        if c == '.' {
            for child in self.children.values() {
                if Self::search(child, &s[1..]) {
                    return true;
                }
            }
        } else {
            if let Some(child) = self.children.get(&c) {
                return Self::search(&child, &s[1..]);
            } else {
                return false;
            }
        }
        false
    }
}


struct WordDictionary {
    trie: Trie,
}

impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            trie: Trie::default(),
        }
    }

    fn add_word(&mut self, word: String) {
        self.trie.insert(&word);
    } 

    fn search(&self, word: String) -> bool {
        self.trie.search(&word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut word_dict: WordDictionary = WordDictionary::new();
        word_dict.add_word("bad".to_string());
        word_dict.add_word("dad".to_string());
        word_dict.add_word("mad".to_string());
        assert_eq!(word_dict.search("pad".to_string()), false);
        assert_eq!(word_dict.search("bad".to_string()), true);
        assert_eq!(word_dict.search(".ad".to_string()), true);
        assert_eq!(word_dict.search("b..".to_string()), true);
    }
}