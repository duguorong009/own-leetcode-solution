fn main() {
    let numbers: Vec<i32> = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(two_sum(numbers, target), vec![1, 2]);
}

fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut l = 0;
    let mut r = numbers.len() - 1;
    while l <= r {
        let sum = numbers[l] + numbers[r];
        if sum == target {
            break;
        }
        if sum < target {
            l += 1;
        }

        if sum > target {
            r -= 1;
        }
    }
    vec![(l + 1) as i32, (r + 1) as i32]
}

#[cfg(test)]
mod tests {
    use crate::two_sum;

    #[test]
    fn test_1() {
        let numbers: Vec<i32> = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(numbers, target), vec![1, 2]);
    }

    #[test]
    fn test_2() {
        let numbers: Vec<i32> = vec![2, 3, 4];
        let target = 6;
        assert_eq!(two_sum(numbers, target), vec![1, 3]);
    }

    #[test]
    fn test_3() {
        let numbers: Vec<i32> = vec![-1, 0];
        let target = -1;
        assert_eq!(two_sum(numbers, target), vec![1, 2]);
    }
}
