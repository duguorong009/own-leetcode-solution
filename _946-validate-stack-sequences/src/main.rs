fn main() {
    println!("Hello, world!");
}

fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
    let mut stack = vec![];
    let mut it = popped.iter().peekable();
    for x in pushed {
        stack.push(x);
        while let (Some(&a), Some(&&b)) = (stack.last(), it.peek()) {
            if a == b {
                stack.pop();
                it.next();
            } else {
                break;
            }
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let pushed = vec![1, 2, 3, 4, 5];
        let popped = vec![4, 5, 3, 2, 1];
        assert!(validate_stack_sequences(pushed, popped));
    }
    #[test]
    fn test_2() {
        let pushed = vec![1, 2, 3, 4, 5];
        let popped = vec![4, 5, 3, 3, 1, 2];
        assert!(validate_stack_sequences(pushed, popped) == false);
    }
}