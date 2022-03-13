use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
    let mut hm: HashMap<&str, usize> = HashMap::new();
    let mut min = usize::MAX;
    let mut res: Vec<String> = vec![];
    for i in 0..list1.len() {
        hm.insert(&list1[i], i);
    }
    for j in 0..list2.len() {
        if let Some(i) = hm.get(list2[j].as_str()) {
            let sum = i + j;
            if sum < min {
                min = sum;
                res.clear();
            }
            if sum == min {
                res.push(list2[j].to_string());
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let list1 = vec!["Shogun".to_string(), "Tapioca Express".to_string(), "Burger King".to_string(), "KFC".to_string()];
        let list2 = vec!["Piatti".to_string(), "The Grill at Torrey Pines".to_string(), "Hungry Hunter Steakhouse".to_string(), "Shogun".to_string()];
        assert_eq!(find_restaurant(list1, list2), vec!["Shogun".to_string()]);
    }

    #[test]
    fn test_2() {
        let list1 = vec!["Shogun".to_string(), "Tapioca Express".to_string(), "Burger King".to_string(), "KFC".to_string()];
        let list2 = vec!["KFC".to_string(), "Shogun".to_string(), "Burger King".to_string()];
        assert_eq!(find_restaurant(list1, list2), vec!["Shogun".to_string()]);
    }
}