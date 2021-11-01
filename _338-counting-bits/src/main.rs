fn main() {
    println!("Hello, world!");
}

fn count_bits(n: i32) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    for i in 0..=n {
        res.push(i.count_ones() as i32);
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::count_bits;

    #[test]
    fn test_1() {
        let n = 2;
        assert_eq!(count_bits(n), vec![0, 1, 1]);
    }

    #[test]
    fn test_2() {
        let n = 5;
        assert_eq!(count_bits(n), vec![0, 1, 1, 2, 1, 2]);
    }
}
