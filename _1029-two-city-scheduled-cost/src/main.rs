fn main() {
    println!("Hello, world!");
}

fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
    let n = costs.len();
    let mut diffs: Vec<i32> = costs.iter().map(|v| v[0] - v[1]).collect();
    diffs.sort_unstable();
    let sum_of_b: i32 = costs.iter().map(|v| v[1]).sum();
    let sum_of_diff: i32 = diffs.iter().take(n / 2).sum();
    sum_of_b + sum_of_diff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let costs = vec![vec![10, 20], vec![30, 200], vec![400, 50], vec![30, 20]];
        assert_eq!(two_city_sched_cost(costs), 110);
    }

    #[test]
    fn test_2() {
        let costs = vec![vec![259, 770], vec![448, 54], vec![926, 667], vec![184, 139], vec![840, 118], vec![577, 469]];
        assert_eq!(two_city_sched_cost(costs), 1859);
    }

    #[test]
    fn test_3() {
        let costs = vec![vec![515, 563], vec![451, 713], vec![537, 709], vec![343, 819], vec![855, 779], vec![457, 60], vec![650, 359], vec![631, 421]];
        assert_eq!(two_city_sched_cost(costs), 3086);
    }
}