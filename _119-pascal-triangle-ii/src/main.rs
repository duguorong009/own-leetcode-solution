fn main() {
    println!("Hello, world!");
}

fn get_row(row_index: i32) -> Vec<i32> {
    let mut res: Vec<Vec<i32>> = vec![];
    for i in 0..=row_index {
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
    println!("{:?}", res);
    res.pop().unwrap_or(vec![])
}

#[cfg(test)]
mod tests {
    use crate::get_row;

    #[test]
    fn test_1() {
        let row_index = 3;
        assert_eq!(get_row(row_index), vec![1, 3, 3, 1]);
    }

    #[test]
    fn test_2() {
        let row_index = 0;
        assert_eq!(get_row(row_index), vec![1]);
    }

    #[test]
    fn test_3() {
        let row_index = 1;
        assert_eq!(get_row(row_index), vec![1, 1]);
    }
}
