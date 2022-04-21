fn main() {
    println!("Hello, world!");
}

struct MyHashSet {
    hs: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {

    fn new() -> Self {
        MyHashSet { hs: vec![] }
    }
    
    fn add(&mut self, key: i32) {
        if !self.contains(key) {
            self.hs.push(key);
        }
    }
    
    fn remove(&mut self, key: i32) {
        let res = self.hs.iter().enumerate().find(|(id, v)| v == &&key);
        if let Some((id, _)) = res {
            self.hs.remove(id);
        }
    }
    
    fn contains(&self, key: i32) -> bool {
        self.hs.contains(&key)
    }
}

// /**
//  * Your MyHashSet object will be instantiated and called as such:
//  * let obj = MyHashSet::new();
//  * obj.add(key);
//  * obj.remove(key);
//  * let ret_3: bool = obj.contains(key);
//  */