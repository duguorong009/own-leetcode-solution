use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    println!("Hello, world!");
}

struct KthLargest {
    pq: BinaryHeap<Reverse<i32>>,
    k: usize,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut pq: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let k = k as usize;
        for x in nums {
            pq.push(Reverse(x));
            if pq.len() > k {
                pq.pop();
            }
        }
        KthLargest { pq, k }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.pq.push(Reverse(val));
        if self.pq.len() > self.k {
            self.pq.pop();
        }
        let x = *self.pq.peek().unwrap();
        x.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut largest = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(largest.add(3), 4);
        assert_eq!(largest.add(5), 5);
        assert_eq!(largest.add(10), 5);
        assert_eq!(largest.add(9), 8);
        assert_eq!(largest.add(4), 8);
    }
}