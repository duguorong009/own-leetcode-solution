extern crate rand;
use rand::Rng;
use std::collections::HashSet;
fn main() {
    println!("Hello, world!");
}

struct RandomizedSet {
    set: HashSet<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            set: HashSet::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        self.set.insert(val)
    }

    fn remove(&mut self, val: i32) -> bool {
        self.set.remove(&val)
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let cap = self.set.capacity();
        let random_number = rng.gen_range(0..cap);
        let random_values = self
            .set
            .iter()
            .enumerate()
            .find(|&(id, _)| id == random_number)
            .map(|(_, item)| *item)
            .into_iter()
            .collect::<Vec<i32>>();
        random_values[0]
    }
}
