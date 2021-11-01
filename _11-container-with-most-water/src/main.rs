fn main() {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    assert_eq!(max_area(height), 49);
}

fn max_area(height: Vec<i32>) -> i32 {
    if height.len() <= 1 {
        return 0;
    }
    let mut l = 0 as usize;
    let mut r = height.len() - 1;
    let mut level = 0;
    let mut res = 0;
    while l < r {
        if height[l] < height[r] {
            let lower = height[l];
            level = level.max(lower);
            if res < level * (r - l) as i32 {
                res = level * (r - l) as i32;
            }
            l += 1;
        } else {
            let lower = height[r];
            level = level.max(lower);
            if res < level * (r - l) as i32 {
                res = level * (r - l) as i32;
            }
            r -= 1;
        }
        println!("{} {}: {}, {}", l, r, level, res);
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::max_area;

    #[test]
    fn test_1() {
        let height = vec![1, 1];
        assert_eq!(max_area(height), 1);
    }

    #[test]
    fn test_2() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(max_area(height), 49);
    }

    #[test]
    fn test_3() {
        let height = vec![4, 3, 2, 1, 4];
        assert_eq!(max_area(height), 16);
    }

    #[test]
    fn test_4() {
        let height = vec![1, 2, 1];
        assert_eq!(max_area(height), 2);
    }
}
