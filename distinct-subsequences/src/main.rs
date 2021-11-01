fn main() {
    let s = "rabbbit".to_string();
    let t = "rabbit".to_string();
    println!("{}", num_distinct(s, t));
}

fn num_distinct(s: String, t: String) -> i32 {
    let s = s.split("").collect::<Vec<&str>>();
    let t = t.split("").collect::<Vec<&str>>();
    let mut already_occupied_ids: Vec<Vec<usize>> = vec![];
    // Check if "t" is inside "s".
    // Record the array of ids when found one occurrence.
    // Repeat the process until all "s" is searched.
    0
}

#[cfg(test)]
mod tests {
    use crate::num_distinct;

    #[test]
    fn test_1() {
        let s = "rabbbit".to_string();
        let t = "rabbit".to_string();
        assert_eq!(num_distinct(s, t), 3);
    }

    #[test]
    fn test_2() {
        let s = "babgbag".to_string();
        let t = "bag".to_string();
        assert_eq!(num_distinct(s, t), 5);
    }
}
