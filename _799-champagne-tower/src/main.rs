fn main() {
    println!("Hello, world!");
}

fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
    let mut res: Vec<Vec<f64>> = vec![vec![0_f64; 102]; 102];
    res[0][0] = poured as f64;
    for r in 0..=query_row {
        for c in 0..=r {
            let q = (res[r as usize][c as usize] - 1.0) / 2.0;
            if q > 0.0 {
                res[r as usize + 1][c as usize] += q;
                res[r as usize + 1][c as usize + 1] += q;
            }
        }
    }
    1_f64.min(res[query_row as usize][query_glass as usize])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let poured = 1;
        let query_row = 1;
        let query_glass = 1;
        assert_eq!(champagne_tower(poured, query_row, query_glass), 0.0);
    }

    #[test]
    fn test_2() {
        let poured = 2;
        let query_row = 1;
        let query_glass = 1;
        assert_eq!(champagne_tower(poured, query_row, query_glass), 0.5);
    }

    #[test]
    fn test_3() {
        let poured = 100000009;
        let query_row = 33;
        let query_glass = 17;
        assert_eq!(champagne_tower(poured, query_row, query_glass), 1.0);
    }
}