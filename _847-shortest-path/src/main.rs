use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");
}

fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
    let n = graph.len();
    let adj: Vec<Vec<usize>> = graph
        .into_iter()
        .map(
            |v| v.into_iter().map(|i| i as usize).collect()
        ).collect();

    let mut visited: HashSet<(u32, usize)> = HashSet::new();
    let mut queue: VecDeque<(u32, usize, i32)> = VecDeque::new();
    for i in 0..n {
        visited.insert((1 << i, i));
        queue.push_back((1 << i, i, 0));
    }

    while let Some((bitset, i, d)) = queue.pop_front() {
        if bitset == (1 << n) - 1 {
            return d;
        }
        for &j in &adj[i] {
            let next_bitset = bitset | (1 << j);
            if visited.insert((next_bitset, j)) {
                queue.push_back((next_bitset, j, d + 1));
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let graph = vec![vec![1, 2, 3], vec![0], vec![0], vec![0]];
        assert_eq!(shortest_path_length(graph), 4);
    }

    #[test]
    fn test_2() {
        let graph = vec![vec![1], vec![0, 2, 4], vec![1,3,4], vec![2], vec![1, 2]];
        assert_eq!(shortest_path_length(graph), 4);
    }
}