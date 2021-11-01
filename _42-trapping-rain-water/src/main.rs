fn main() {
    let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    assert_eq!(trap(height), 6);
}

// Main solution to this problem is to
// get left & right boundary
// calculate possible water amount by min
// subract the height of that place.
fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    if n == 0 {
        return 0;
    }
    let mut l = 0;
    let mut r = n - 1;
    let mut level = 0;
    let mut res = 0;
    while l < r {
        if height[l] < height[r] {
            let lower = height[l];
            level = level.max(lower);
            res += level - lower;
            l += 1;
        } else {
            let lower = height[r];
            level = level.max(lower);
            res += level - lower;
            r -= 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::trap;

    #[test]
    fn test_1() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(trap(height), 6);
    }

    #[test]
    fn test_2() {
        let height = vec![4, 2, 0, 3, 2, 5];
        assert_eq!(trap(height), 9);
    }
}
