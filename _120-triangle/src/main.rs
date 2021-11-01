fn main() {
    let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
    assert_eq!(minimum_total(triangle), 11);
}

fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let height = triangle.len();
    if height == 1 {
        return triangle[0][0];
    }
    let mut res: Vec<Vec<i32>> = vec![vec![triangle[0][0]]];
    for i in 1..height {
        res.push(vec![]);
        for j in 0..=i {
            let mut min_sum: i32 = 0;
            if j == 0 {
                min_sum = res[i - 1][j];
            } else if j == i {
                min_sum = res[i - 1][j - 1];
            } else {
                min_sum = res[i - 1][j - 1].min(res[i - 1][j]);
            }
            res[i].push(min_sum + triangle[i][j]);
        }
    }
    println!("{:?}", res);
    *(res[height - 1].iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    use crate::minimum_total;

    #[test]
    fn test_1() {
        let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        assert_eq!(minimum_total(triangle), 11);
    }

    #[test]
    fn test_2() {
        let triangle = vec![vec![-10]];
        assert_eq!(minimum_total(triangle), -10);
    }

    #[test]
    fn test_3() {
        let triangle = vec![vec![-1], vec![-2, -3]];
        assert_eq!(minimum_total(triangle), -4);
    }
}
