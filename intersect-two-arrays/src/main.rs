use std::collections::HashMap;

fn main() {
    let nums1: Vec<i32> = vec![4, 9, 5];
    let nums2: Vec<i32> = vec![9, 4, 9, 8, 4];
    assert_eq!(intersect(nums1, nums2), vec![9, 4]);
}

//
// There are multiple solutions for this problem.
// My approach is to introduce 2 hashmaps to store the element frequencies of two arrays,
// compare the freq between two maps with same key & construct result array.
//
//
// Another approach is to first sort 2 arrays, introduce 2 index(or pointer) to pick the
// elements of 2 arrays, move the pointers according to relation of pointer values of arrays,
// & construct the result.
//

fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut intersect_vec: Vec<i32> = vec![];
    let mut nums1_elem_freq: HashMap<i32, i32> = HashMap::new();
    let mut nums2_elem_freq: HashMap<i32, i32> = HashMap::new();
    for elem in nums1 {
        *(nums1_elem_freq.entry(elem).or_insert(0)) += 1;
    }

    for elem in nums2 {
        *(nums2_elem_freq.entry(elem).or_insert(0)) += 1;
    }

    let search_keys = if nums1_elem_freq.keys().len() < nums2_elem_freq.keys().len() {
        nums1_elem_freq.keys().into_iter()
    } else {
        nums2_elem_freq.keys().into_iter()
    };

    for key in search_keys {
        let min_occurrence = *nums1_elem_freq
            .get(key)
            .unwrap_or(&(-1))
            .min(nums2_elem_freq.get(key).unwrap_or(&(-1)));
        for _i in 0..min_occurrence {
            intersect_vec.push(*key)
        }
    }

    intersect_vec
}

#[cfg(test)]
mod tests {
    use crate::intersect;

    #[test]
    fn test_1() {
        let nums1: Vec<i32> = vec![1, 2, 2, 1];
        let nums2: Vec<i32> = vec![2, 2];
        assert_eq!(intersect(nums1, nums2), vec![2, 2]);
    }

    #[test]
    fn test_2() {
        let nums1: Vec<i32> = vec![4, 9, 5];
        let nums2: Vec<i32> = vec![9, 4, 9, 8, 4];
        assert_eq!(intersect(nums1, nums2), vec![9, 4]);
    }
}
