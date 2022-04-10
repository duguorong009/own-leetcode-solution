fn main() {
    println!("Hello, world!");
}

fn cal_points(ops: Vec<String>) -> i32 {
    let mut res: Vec<i32> = vec![];
    for op in ops {
        if let Ok(v) = op.as_str().parse::<i32>() {
            res.push(v);
        } else {
            let n = res.len();
            match op.as_str() {
                "+" => {
                    res.push(res[n - 1] + res[n - 2]);
                }
                "C" => {
                    res.pop();
                }
                "D" => {
                    res.push(res[n - 1] * 2);
                }
                _ => unreachable!("No more ops except for those")
            }
        }
    }
    res.into_iter().sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ops = vec![
            "5".to_string(),
            "2".to_string(),
            "C".to_string(),
            "D".to_string(),
            "+".to_string(),
        ];
        assert_eq!(cal_points(ops), 30);
    }

    #[test]
    fn test_2() {
        let ops = vec![
            "5".to_string(),
            "-2".to_string(),
            "4".to_string(),
            "C".to_string(),
            "D".to_string(),
            "9".to_string(),
            "+".to_string(),
            "+".to_string(),
        ];
        assert_eq!(cal_points(ops), 27);
    }

    #[test]
    fn test_3() {
        let ops = vec!["1".to_string()];
        assert_eq!(cal_points(ops), 1);
    }
}
