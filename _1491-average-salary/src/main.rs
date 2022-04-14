fn main() {
    println!("Hello, world!");
}

fn average(salary: Vec<i32>) -> f64 {
    let mut salary = salary;
    salary.sort_unstable();

    let n = salary.len();
    salary[1..n - 1]
        .iter()
        .map(|v| *v as f64)
        .sum::<f64>() / (n as f64 - 2.)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let salary = vec![4000, 3000, 1000, 2000];
        assert_eq!(average(salary), 2500.);
    }

    #[test]
    fn test_2() {
        let salary = vec![1000, 2000, 3000];
        assert_eq!(average(salary), 2000.);
    }
}
