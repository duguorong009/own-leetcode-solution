use rand::distributions::WeightedIndex;
use rand::prelude::*;

fn main() {
    println!("Hello, world!");
}

struct Solution {
    dist: WeightedIndex<i32>,
    rng: ThreadRng,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(w: Vec<i32>) -> Self {
        let rng = thread_rng();
        let dist = WeightedIndex::new(w).unwrap();
        Solution { dist, rng }
    }
    
    fn pick_index(&mut self) -> i32 {
        self.rng.sample(&self.dist) as i32
    }
}
