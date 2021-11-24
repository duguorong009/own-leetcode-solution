fn main() {
    let first_list = vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]];
    let second_list = vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]];
    let expected = vec![
        vec![1, 2],
        vec![5, 5],
        vec![8, 10],
        vec![15, 23],
        vec![24, 24],
        vec![25, 25],
    ];
    assert_eq!(interval_intersection(first_list, second_list), expected);
}

fn interval_intersection(first_list: Vec<Vec<i32>>, second_list: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n1 = first_list.len();
    let n2 = second_list.len();
    if n1 == 0 || n2 == 0 {
        return vec![];
    }
    let mut res: Vec<Vec<i32>> = vec![];
    let mut first_ptr = 0_usize;
    let mut second_ptr = 0_usize;
    let n1 = first_list.len();
    let n2 = second_list.len();
    while first_ptr < n1 && second_ptr < n2 {
        if second_list[second_ptr][0] <= first_list[first_ptr][1]
            && first_list[first_ptr][0] <= second_list[second_ptr][1]
        {
            res.push(vec![
                first_list[first_ptr][0].max(second_list[second_ptr][0]),
                first_list[first_ptr][1].min(second_list[second_ptr][1]),
            ]);
        }

        // Now increment either first_prt or second_ptr conditionally
        if first_list[first_ptr][1] > second_list[second_ptr][1] {
            second_ptr += 1;
        } else {
            first_ptr += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let first_list = vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]];
        let second_list = vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]];
        let expected = vec![
            vec![1, 2],
            vec![5, 5],
            vec![8, 10],
            vec![15, 23],
            vec![24, 24],
            vec![25, 25],
        ];
        assert_eq!(interval_intersection(first_list, second_list), expected);
    }

    #[test]
    fn test_2() {
        let first_list = vec![vec![1, 3], vec![5, 9]];
        let second_list: Vec<Vec<i32>> = vec![];
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(interval_intersection(first_list, second_list), expected);
    }

    #[test]
    fn test_3() {
        let first_list = vec![];
        let second_list: Vec<Vec<i32>> = vec![vec![4, 8], vec![10, 12]];
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(interval_intersection(first_list, second_list), expected);
    }

    #[test]
    fn test_4() {
        let first_list = vec![vec![1, 7]];
        let second_list: Vec<Vec<i32>> = vec![vec![3, 10]];
        let expected: Vec<Vec<i32>> = vec![vec![3, 7]];
        assert_eq!(interval_intersection(first_list, second_list), expected);
    }
}
