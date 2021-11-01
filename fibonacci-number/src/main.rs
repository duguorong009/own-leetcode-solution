fn main() {
    println!("{}", fib(2));
}

fn fib(n: i32) -> i32 {
    let mut fib_cache: Vec<i32> = vec![0, 1];
    for i in 2..n + 1 {
        fib_cache.push(fib_cache[(i - 1) as usize] + fib_cache[(i - 2) as usize]);
    }
    fib_cache[n as usize]
}

#[cfg(test)]
mod tests {
    use crate::fib;

    #[test]
    fn test_1() {
        assert_eq!(fib(2), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(fib(3), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(fib(4), 3);
    }
}
