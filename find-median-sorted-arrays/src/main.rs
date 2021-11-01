// This problem needs the Binary search for solution.
// Binary search provides O(log(m+n)) time complexity.
// Also, binary search approach is possible since 2 arrays are sorted ones.

/*
1. Finding a pivot point where all elements to the left are smaller  and
   all the elements to the right are greater, you can find the median.
   x x [x] | [x] x x  llong rlong
     y [y] | [y] y    lshort rshort
   x x y [y] [x] | [y] y [x] x x

2. Any pivot point in the smaller array has a corresponding point on the
    large array, that divides the total # of elements in two
    x x x x x x
      y y y y

3. After picking a pivot point, it's possible to determine whether we need to
    go left or right
    1 2 3 | 4 5 6
      2 3 | 4 5

4. Code outline
    - Binary search on small array
    - getIndices: m -> llong, rlong etc
    - getDirection: llong , rlong, etc -> get direction(left or right)
    - getResult : rlong, llong, etc -> calculate the median
*/
fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let n1 = nums1.len();
    let n2 = nums2.len();
    if n1 < n2 {
        return find_median_sorted_arrays(nums2, nums1);
    }
    let mut lo = 0;
    let mut hi = n2 * 2;
    while lo <= hi {
        let mid2 = (lo + hi) / 2;
        let mid1 = n1 + n2 - mid2;
        let l1 = if mid1 == 0 {
            std::i32::MIN
        } else {
            nums1[(mid1 - 1) / 2]
        };
        let l2 = if mid2 == 0 {
            std::i32::MIN
        } else {
            nums2[(mid2 - 1) / 2]
        };
        let r1 = if mid1 == n1 * 2 {
            std::i32::MAX
        } else {
            nums1[mid1 / 2]
        };
        let r2 = if mid2 == n2 * 2 {
            std::i32::MAX
        } else {
            nums2[mid2 / 2]
        };

        if l1 > r2 {
            lo = mid2 + 1;
        } else if l2 > r1 {
            hi = mid2 - 1;
        } else {
            return (l1.max(l2) + r1.min(r2)) as f64 / 2.0;
        }
    }
    panic!()
}

fn main() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];

    println!("{}", find_median_sorted_arrays(nums1, nums2));
}

#[cfg(test)]
mod tests {
    use crate::find_median_sorted_arrays;

    #[test]
    fn test_1() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.00);
    }

    #[test]
    fn test_2() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.5);
    }

    #[test]
    fn test_3() {
        let nums1 = vec![0, 0];
        let nums2 = vec![0, 0];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 0.00);
    }

    #[test]
    fn test_4() {
        let nums1 = vec![];
        let nums2 = vec![1];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 1.00);
    }

    #[test]
    fn test_5() {
        let nums1 = vec![2];
        let nums2 = vec![];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.00);
    }
}
