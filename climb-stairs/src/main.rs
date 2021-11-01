fn main() {
    let n = 2;
    println!("{}", climb_stairs(n));
}

fn climb_stairs(n: i32) -> i32 {
    let mut climb_ways: Vec<i32> = vec![1, 1];
    for i in 2..n + 1 {
        climb_ways.push(climb_ways[(i - 1) as usize] + climb_ways[(i - 2) as usize]);
    }
    climb_ways[n as usize]

    // / Solution 2. Use rust iterator function "fold".
    // match n {
    //     1 | 2 => n,
    //     k => (2..k).fold((1, 2), |acc, _| (acc.1, acc.0 + acc.1)).1,
    // }
}

#[cfg(test)]
mod tests {
    use crate::climb_stairs;

    #[test]
    fn test_1() {
        let n: i32 = 2;
        assert_eq!(climb_stairs(n), 2);
    }

    #[test]
    fn test_2() {
        let n: i32 = 3;
        assert_eq!(climb_stairs(n), 3);
    }
}
