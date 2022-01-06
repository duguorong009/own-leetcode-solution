fn main() {
    println!("Hello, world!");
}

fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
    let mut pairs: Vec<(i32, i32)> = vec![];
    for trip in trips {
        pairs.push((trip[1], trip[0]));
        pairs.push((trip[2], -trip[0]));
    }
    pairs.sort_unstable();
    let mut max = 0;
    let mut count = 0;
    for pair in pairs {
        count += pair.1;
        max = max.max(count);
    }
    max <= capacity
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let trips = vec![vec![2, 1, 5], vec![3, 3, 7]];
        let capacity = 4;
        assert_eq!(car_pooling(trips, capacity), false);
    }

    #[test]
    fn test_2() {
        let trips = vec![vec![2, 1, 5], vec![3, 3, 7]];
        let capacity = 5;
        assert_eq!(car_pooling(trips, capacity), true);
    }
}
