fn main() {
    println!("Hello, world!");
}

fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let wealths = accounts
        .into_iter()
        .map(|banks| banks.iter().sum())
        .collect::<Vec<i32>>();
    wealths.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let accounts = vec![vec![1, 2,3], vec![3, 2, 1]];
        assert_eq!(maximum_wealth(accounts), 6);
    }

    #[test]
    fn test_2() {
        let accounts = vec![vec![1, 5], vec![7, 3], vec![3, 5]];
        assert_eq!(maximum_wealth(accounts), 10);
    }

    #[test]
    fn test_3() {
        let accounts = vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]];
        assert_eq!(maximum_wealth(accounts), 17);
    }
}