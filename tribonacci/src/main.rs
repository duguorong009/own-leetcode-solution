fn main() {
    println!("{}", tribonacci(25));
}

fn tribonacci(n: i32) -> i32 {
    let mut tribonacci_cache: Vec<i32> = vec![0, 1, 1];
    for i in 3..n + 1 {
        tribonacci_cache.push(
            tribonacci_cache[(i - 1) as usize]
                + tribonacci_cache[(i - 2) as usize]
                + tribonacci_cache[(i - 3) as usize],
        );
    }
    tribonacci_cache[n as usize]
}

#[cfg(test)]
mod tests {
    use crate::tribonacci;

    #[test]
    fn test_1() {
        assert_eq!(tribonacci(4), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(tribonacci(25), 1389537);
    }
}
