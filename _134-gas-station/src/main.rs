fn main() {
    println!("Hello, world!");
}

fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let n = gas.len();

    let mut total = 0;
    let mut cur = 0;
    let mut idx = 0;

    for i in 0..n {
        total += gas[i] - cost[i];
        cur += gas[i] - cost[i];
        if cur < 0 {
            idx = i + 1;
            cur = 0;
        }
    }

    if total >= 0 {
        idx as i32
    } else {
        -1
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let gas= vec![1, 2, 3, 4, 5];
        let cost = vec![3, 4, 5, 1, 2];
        assert_eq!(can_complete_circuit(gas, cost), 3);
    }

    #[test]
    fn test_2() {
        let gas = vec![2, 3, 4];
        let cost = vec![3, 4, 3];
        assert_eq!(can_complete_circuit(gas, cost), -1);
    }
}