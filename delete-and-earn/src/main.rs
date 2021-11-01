use std::collections::HashMap;

fn main() {
    let nums = vec![3, 4, 2];
    println!("{}", delete_and_earn(nums));
}

fn delete_and_earn(nums: Vec<i32>) -> i32 {
    let mut num_freq: HashMap<i32, i32> = HashMap::new();
    let mut elems: Vec<i32> = vec![];
    for elem in nums {
        *(num_freq.entry(elem).or_insert(0)) += elem;
        elems.push(elem);
    }
    elems.sort();
    let mut individual_profit: Vec<i32> = vec![];
    for i in elems[0]..=elems[elems.len() - 1] {
        individual_profit.push(*num_freq.get(&i).unwrap_or(&0));
    }
    if individual_profit.len() == 1 {
        return individual_profit[0];
    }

    let mut max_profit: Vec<i32> = vec![
        individual_profit[0],
        individual_profit[0].max(individual_profit[1]),
    ];
    for i in 2..individual_profit.len() {
        max_profit.push((individual_profit[i] + max_profit[i - 2]).max(max_profit[i - 1]));
    }
    max_profit[max_profit.len() - 1]
}

#[cfg(test)]
mod tests {
    use crate::delete_and_earn;

    #[test]
    fn test_1() {
        let nums = vec![3, 4, 2];
        assert_eq!(delete_and_earn(nums), 6);
    }

    #[test]
    fn test_2() {
        let nums = vec![2, 2, 3, 3, 3, 4];
        assert_eq!(delete_and_earn(nums), 9);
    }

    #[test]
    fn test_3() {
        let nums = vec![2, 2];
        assert_eq!(delete_and_earn(nums), 4);
    }
}
