fn main() {
    println!("Hello, world!");
}

fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates;
    candidates.sort_unstable();
    let n = candidates.len();

    let mut combination: Vec<i32> = vec![];
    let mut res: Vec<Vec<i32>> = vec![];
    backtrack(0, target, &mut combination, &mut res, &candidates, n);
    res
}

fn backtrack(
    start: usize, 
    target: i32, 
    combination: &mut Vec<i32>, 
    all: &mut Vec<Vec<i32>>, 
    candidates: &[i32], 
    n: usize
) {
    if target == 0 {
        all.push(combination.to_vec());
    } else {
        for i in start..n {
            if candidates[i] > target {
                break;
            } else {
                combination.push(candidates[i]);
                backtrack(i, target - candidates[i], combination, all, candidates, n);
                combination.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        assert_eq!(combination_sum(candidates, target), vec![vec![2, 2, 3], vec![7]]);
    }

    #[test]
    fn test_2(){
        let candidates = vec![2, 3, 5];
        let target = 8;
        assert_eq!(combination_sum(candidates, target), vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]);
    }

    #[test]
    fn test_3() {
        let candidates = vec![2];
        let target = 1;
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(combination_sum(candidates, target), expected);
    }
}