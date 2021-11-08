fn main() {
    println!("Hello, world!");
}

fn num_trees(n: i32) -> i32 {
    let n = n as usize;
    if n < 2 {
        return 1;
    }
    if n == 2 {
        return 2;
    }
    let mut res: Vec<i32> = vec![1, 1, 2];
    for i in 3..=n {
        let mut catalan_i = 0;
        for j in 1..=i {
            catalan_i += res[j - 1] * res[i - j];
        }
        res.push(catalan_i);
    }
    res[n]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let n = 3;
        assert_eq!(num_trees(n), 5);
    }

    #[test]
    fn test_2() {
        let n = 1;
        assert_eq!(num_trees(n), 1);
    }

    #[test]
    fn test_3() {
        let n = 4;
        assert_eq!(num_trees(n), 14);
    }

    #[test]
    fn test_4() {
        let n = 5;
        assert_eq!(num_trees(n), 42);
    }
}
