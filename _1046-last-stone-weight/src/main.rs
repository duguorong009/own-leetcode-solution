fn main() {
    println!("Hello, world!");
}

fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut stones = stones;
    while stones.len() > 1 {
        stones.sort_unstable();
        let biggest = stones.pop().unwrap();
        let second_biggest = stones.pop().unwrap();
        let diff = biggest - second_biggest;
        if diff != 0 {
            stones.push(diff);
        }
    }
    if stones.is_empty() {
        0
    } else {
        stones[0]
    }
}

// // binaryheap solution
// fn last_stone_weight(stones: Vec<i32>) -> i32 {
//     let mut pq: BinaryHeap<i32> = BinaryHeap::from(stones);
//     while let Some(a) = pq.pop() {
//         if let Some(b) = pq.pop() {
//             if a - b != 0 {
//                 pq.push(a - b);
//             }
//         } else {
//             return a;
//         }
//     }
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let stones = vec![2, 7, 4, 1, 8, 1];
        assert_eq!(last_stone_weight(stones), 1);
    }

    #[test]
    fn test_2() {
        let stones = vec![1];
        assert_eq!(last_stone_weight(stones), 1);
    }
}