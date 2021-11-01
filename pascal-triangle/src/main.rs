fn main() {
    println!("{:?}", generate(5));
}

fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    for i in 0..num_rows {
        let ui = i as usize;
        res.push(vec![]);
        for j in 0..=i {
            let uj = j as usize;
            if j == 0 || j == i {
                res[ui].push(1);
            } else {
                let prev = &res[ui - 1];
                let sum = prev[uj - 1] + prev[uj];
                res[ui].push(sum);
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::generate;

    #[test]
    fn test_1() {
        let num_rows = 5;
        let expectation = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];
        assert_eq!(generate(num_rows), expectation);
    }

    #[test]
    fn test_2() {
        let num_rows = 1;
        let expectation = vec![vec![1]];
        assert_eq!(generate(num_rows), expectation);
    }
}
