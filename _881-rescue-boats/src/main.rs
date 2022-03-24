use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
    let n = people.len();
    let mut people = people;
    people.sort_unstable();
    people.reverse();
    let mut i = 0;
    let mut j = n - 1;
    let mut res = 0;
    while i <= j {
        res += 1;
        if people[i] + people[j] <= limit {
            j -= 1;
        }
        i += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_1() {
        let people = vec![1, 2];
        let limit = 3;
        assert_eq!(rescue_boats(people, limit), 1);
    }

    #[test]
    fn test_2() {
        let people = vec![3, 2, 2, 1];
        let limit = 3;
        assert_eq!(rescue_boats(people, limit), 3);
    }

    #[test]
    fn test_3() {
        let people = vec![3, 5, 3, 4];
        let limit = 5;
        assert_eq!(rescue_boats(people, limit), 4);
    }

    #[test]
    fn test_4() {
        let people = vec![5, 1, 4, 2];
        let limit = 6;
        assert_eq!(rescue_boats(people, limit), 2);
    }
}