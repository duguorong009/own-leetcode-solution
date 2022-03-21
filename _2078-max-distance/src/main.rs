fn main() {
    println!("Hello, world!");
}

fn max_distance(colors: Vec<i32>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let colors = vec![1, 1, 1, 6, 1, 1, 1];
        assert_eq!(max_distance(colors), 3);
    }

    #[test]
    fn test_2() {
        let colors = vec![1, 8, 3, 8, 3];
        assert_eq!(max_distance(colors), 4);
    }

    #[test]
    fn test_3() {
        let colors = vec![0, 1];
        assert_eq!(max_distance(colors), 1);
    }
}