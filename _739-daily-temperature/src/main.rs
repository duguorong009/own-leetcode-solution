fn main() {
    println!("Hello, world!");
}

fn daily_temperature(temperatures: Vec<i32>) -> Vec<i32> {
    let n = temperatures.len();
    let mut stack: Vec<(i32, usize)> = vec![];
    let mut res: Vec<i32> = vec![0; n];
    for (i, &t) in temperatures.iter().enumerate() {
        while stack.len() != 0 && t > stack[stack.len() - 1].0 {
            let (_, stack_id) = stack.pop().unwrap();
            res[stack_id] = (i - stack_id) as i32;
        }
        stack.push((t, i));
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
        assert_eq!(
            daily_temperature(temperatures),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }

    #[test]
    fn test_2() {
        let temperatures = vec![30, 40, 50, 60];
        assert_eq!(daily_temperature(temperatures), vec![1, 1, 1, 0]);
    }

    #[test]
    fn test_3() {
        let temperatures = vec![30, 60, 90];
        assert_eq!(daily_temperature(temperatures), vec![1, 1, 0]);
    }
}
