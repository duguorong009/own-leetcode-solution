fn main() {
    println!("Hello, world!");
}

fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
    let n = position.len();
    let mut a = 0;
    let mut b = 0;
    for i in 0..n {
        if position[i] % 2 == 0 {
            a += 1;
        } else {
            b += 1;
        }
    }
    a.min(b)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let position = vec![1, 2, 3];
        assert_eq!(min_cost_to_move_chips(position), 1);
    }

    #[test]
    fn test_2() {
        let position = vec![2, 2, 2, 3, 3];
        assert_eq!(min_cost_to_move_chips(position), 2);
    }

    #[test]
    fn test_3() {
        let position = vec![1, 1000000000];
        assert_eq!(min_cost_to_move_chips(position), 1);
    }

    #[test]
    fn test_4() {
        let position = vec![2, 3, 3];
        assert_eq!(min_cost_to_move_chips(position), 1);
    }
}
