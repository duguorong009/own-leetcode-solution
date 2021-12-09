use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");
}

fn can_reach(arr: Vec<i32>, start: i32) -> bool {
    let n = arr.len();
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    queue.push_back(start);
    while let Some(i) = queue.pop_front() {
        if arr[i as usize] == 0 {
            return true;
        } else {
            let l = i - arr[i as usize];
            let r = i + arr[i as usize];
            if l >= 0 && !visited[l as usize] {
                visited[l as usize] = true;
                queue.push_back(l);
            }

            if r < n as i32 && !visited[r as usize] {
                visited[r as usize] = true;
                queue.push_back(r);
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let arr = vec![4, 2, 3, 0, 3, 1, 2];
        let start = 5;
        assert_eq!(can_reach(arr, start), true);
    }

    #[test]
    fn test_2() {
        let arr = vec![4, 2, 3, 0, 3, 1, 2];
        let start = 0;
        assert_eq!(can_reach(arr, start), true);
    }

    #[test]
    fn test_3() {
        let arr = vec![3, 0, 2, 1, 2];
        let start = 2;
        assert_eq!(can_reach(arr, start), false);
    }
}
