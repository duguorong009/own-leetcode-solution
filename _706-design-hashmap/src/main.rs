fn main() {
    println!("Hello, world!");
}

struct MyHashMap {
    v: Vec<i32>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {

    fn new() -> Self {
        MyHashMap {
            v: vec![-1; 1_000_000 + 1],
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.v[key as usize] = value;
    }

    fn get(&self, key: i32) -> i32 {
        self.v[key as usize]
    }

    fn remove(&mut self, key: i32) {
        self.v[key as usize] = -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut hm = MyHashMap::new();
        hm.put(1, 1);
        hm.put(2, 2);
        assert_eq!(hm.get(1), 1);
        assert_eq!(hm.get(3), -1);
        hm.put(2, 1);
        assert_eq!(hm.get(2), 1);
        hm.remove(2);
        assert_eq!(hm.get(2), -1);
    }
}