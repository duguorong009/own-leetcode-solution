fn main() {
    let cost = vec![10, 15, 20];
    println!("{}", min_cost_climbing_stairs(cost));
}

fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    // // Greedy solution.
    // // Principle is to change every element as element + min(prev_elem, prev_prev_elem).
    // // Drawback is that the array should be mutable for this operation.
    // let mut cost = cost;
    // for i in 2..cost.len() {
    //     cost[i] += cost[i - 1].min(cost[i - 2]);
    // }
    // cost[cost.len() - 1].min(cost[cost.len() - 2])

    // DP-like solution
    let mut step1: i32 = 0;
    let mut step2: i32 = 0;
    for i in (0..cost.len()).rev() {
        print!("{}th round: ", i);
        let current_step = cost[i] + step1.min(step2);
        step1 = step2;
        step2 = current_step;
        println!("{}, {}, {}", step1, step2, current_step);
    }

    step1.min(step2)
}

#[cfg(test)]
mod tests {
    use crate::min_cost_climbing_stairs;

    #[test]
    fn test_1() {
        let cost: Vec<i32> = vec![10, 15, 20];
        assert_eq!(min_cost_climbing_stairs(cost), 15);
    }

    #[test]
    fn test_2() {
        let cost: Vec<i32> = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        assert_eq!(min_cost_climbing_stairs(cost), 6);
    }
}
