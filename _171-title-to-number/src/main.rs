fn main() {
    println!("Hello, world!");
}

fn title_to_number(column_title: String) -> i32 {
    let mut res = 0;
    for c in column_title.chars() {
        res *= 26;
        res += c as u32 - 'A' as u32 + 1;
    }
    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let column_title = "A".to_string();
        assert_eq!(title_to_number(column_title), 1);
    }

    #[test]
    fn test_2() {
        let column_title = "AB".to_string();
        assert_eq!(title_to_number(column_title), 28);
    }

    #[test]
    fn test_3() {
        let column_title = "ZY".to_string();
        assert_eq!(title_to_number(column_title), 701);
    }
}