use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");
}

fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    if n == 1 {
        return vec![0];
    }
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    let mut visited: Vec<bool> = vec![false; n];
    let mut degree: Vec<usize> = vec![0; n];

    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        graph[u].push(v);
        graph[v].push(u);
        degree[u] += 1;
        degree[v] += 1;
    }

    let mut leaves: VecDeque<usize> = VecDeque::new();
    for i in 0..n {
        if graph[i].len() == 1 {
            leaves.push_back(i);
        }
    }
    let mut m = n;
    while m > 2 {
        m -= leaves.len();
        for _ in 0..leaves.len() {
            let u = leaves.pop_front().unwrap();
            visited[u] = true;
            for &v in &graph[u] {
                if !visited[v] {
                    degree[v] -= 1;
                    if degree[v] == 1 {
                        leaves.push_back(v);
                    }
                }
            }
        }
    }
    leaves.into_iter().map(|x| x as i32).collect()
}

#[cfg(test)]
mod tests {
    use crate::find_min_height_trees;

    #[test]
    fn test_1() {
        let n = 4;
        let edges = vec![vec![1, 0], vec![1, 2], vec![1, 3]];
        assert_eq!(find_min_height_trees(n, edges), vec![1]);
    }

    #[test]
    fn test_2() {
        let n = 6;
        let edges = vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]];
        assert_eq!(find_min_height_trees(n, edges), vec![3, 4]);
    }

    #[test]
    fn test_3() {
        let n = 2;
        let edges = vec![vec![0, 1]];
        assert_eq!(find_min_height_trees(n, edges), vec![0, 1]);
    }

    #[test]
    fn test_4() {
        let n = 1;
        let edges = vec![];
        assert_eq!(find_min_height_trees(n, edges), vec![0]);
    }
}
