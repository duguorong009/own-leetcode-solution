fn main() {
    println!("Hello, world!");
}

fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
    let mut first: Option<usize> = None;
    let mut last: Option<usize> = None;
    let mut prev: Option<usize> = None;
    let n = seats.len();
    let mut max = 0;
    for i in 0..n {
        if seats[i] == 1 {
            if first.is_none() {
                first = Some(i);
            }
            if let Some(j) = prev {
                max = usize::max(( i - j) / 2, max);
            }
            prev  = Some(i);
            last = Some(i);
        }
    }

    max = usize::max(first.unwrap(), max);
    max = usize::max(n - 1 - last.unwrap(), max);
    max as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let seats = vec![1, 0, 0, 0, 1, 0, 1];
        assert_eq!(max_dist_to_closest(seats), 2);
    }

    #[test]
    fn test_2() {
        let seats = vec![1, 0, 0, 0];
        assert_eq!(max_dist_to_closest(seats), 3)
    }

    #[test]
    fn test_3() {
        let seats = vec![0, 1];
        assert_eq!(max_dist_to_closest(seats), 1);
    }

    #[test]
    fn test_4() {
        let seats = vec![0, 0, 1, 0];
        assert_eq!(max_dist_to_closest(seats), 2);
    }
}