use std::collections::{VecDeque, HashMap};

fn main() {
    println!("Hello, world!");
}
/// Given an array of integers `arr`, you are initially positioned 
/// at the first index of the array.
/// In one step you can jump from index `i` to index:
/// `i + 1` where: i + 1 < arr.length
/// `i - 1` where: i - 1 >= 0
/// `j` where: arr[i] = arr[j] and i != j
/// Return the minimum number of steps to reach the last index of array.
/// Notice that you cannot jump outside of the array at any time.
/// 
/// Solution : Apply the BFS(Breadth-first-search) using `queue`.
pub fn min_jumps(arr: Vec<i32>) -> i32 {

    if arr.len() == 1 {
        return 0;
    }

    let mut jumps = 0;
    let n = arr.len();
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(0);
    let mut visited: Vec<usize> = vec![0];
    let mut indexes: HashMap<i32, Vec<usize>> = HashMap::new();
    for (i, &num) in arr.iter().enumerate() {
        indexes.entry(num).or_insert(vec![i]).push(i);    
    }

    while queue.len() > 0 {
        for _ in 0..queue.len() {
            let next_id = queue.pop_front().unwrap();
            if next_id == n - 1 {
                return jumps;
            }
            let next_id_val = arr[next_id];
            let val_ids = indexes.get(&next_id_val).unwrap();
            let mut next_ids: Vec<usize> = if next_id == 0 { vec![next_id + 1]} else {vec![next_id - 1, next_id + 1]};
            for &id in val_ids {
                next_ids.push(id);
            }
            for id in next_ids {
                if visited.contains(&id) || (id >= arr.len()) {
                   continue;
                }
                visited.push(id);
                queue.push_back(id);
            }
        }
        jumps += 1;
    }
    jumps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let arr = vec![100, -23, -23, 404, 100, 23, 23, 23,3, 404 ];
        assert_eq!(min_jumps(arr), 3);
    }

    #[test]
    fn test_2() {
        let arr = vec![7];
        assert_eq!(min_jumps(arr), 0);
    }

    #[test]
    fn test_3() {
        let arr = vec![7, 6, 9, 6, 9, 6, 9, 7 ];
        assert_eq!(min_jumps(arr), 1);
    }
}