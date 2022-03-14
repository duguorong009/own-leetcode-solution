fn main() {
    println!("Hello, world!");
}

fn min_moves(target: i32, max_doubles: i32) -> i32 {
    let mut target = target;
    let mut max_doubles = max_doubles;
    let mut count = 0;
    while target != 1 {
        if max_doubles == 0 {
            break;
        }
        if target % 2 == 1 {
            target -= 1;
            count += 1;
        } else {
            target /= 2;
            max_doubles -= 1;
            count += 1;
        }
    }
    if target == 1 {
        count
    } else {
        count + target - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_moves() {
        assert_eq!(min_moves(5, 0), 4);
        assert_eq!(min_moves(19, 2), 7);
        assert_eq!(min_moves(10, 4), 4);
    }

    #[test]
    fn test_min_moves_ex() {
        assert_eq!(min_moves(1, 100), 0);
    }
}