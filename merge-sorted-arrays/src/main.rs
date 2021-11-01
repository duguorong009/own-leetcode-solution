fn main() {
    let mut nums1 = vec![2, 0];
    let m = 1;
    let mut nums2 = vec![1];
    let n = 1;
    merge(&mut nums1, m, &mut nums2, n);
    assert_eq!(nums1, vec![1, 2]);
}

fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    // // Rust-way
    // nums1.truncate(m as usize);
    // nums2.truncate(n as usize);
    // nums1.append(nums2);
    // nums1.sort_unstable();

    if n == 0 {
        return;
    } else if m == 0 {
        for i in 0..nums2.len() {
            nums1[i] = nums2[i];
        }
        return;
    } else {
        let mut nums1_ptr = m - 1;
        let mut nums2_ptr = n - 1;
        loop {
            if nums1_ptr == -1 && nums2_ptr == -1 {
                break;
            }
            let n1 = if nums1_ptr != -1 {
                nums1[nums1_ptr as usize]
            } else {
                std::i32::MIN
            };
            let n2 = if nums2_ptr != -1 {
                nums2[nums2_ptr as usize]
            } else {
                std::i32::MIN
            };
            if n1 > n2 {
                nums1[(nums1_ptr + nums2_ptr + 1) as usize] = n1;
                nums1_ptr -= 1;
            } else {
                nums1[(nums1_ptr + nums2_ptr + 1) as usize] = n2;
                nums2_ptr -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::merge;

    #[test]
    fn test_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_2() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2: Vec<i32> = vec![];
        let n = 0;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_3() {
        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }
}
