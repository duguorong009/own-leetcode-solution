fn main() {
    let s = "PAYPALISHIRING".to_string();
    let num_rows = 3;
    assert_eq!(convert(s, num_rows), "PAHNAPLSIIGYIR".to_string());
}

// Solution:
//      i
//      0  P        I       N
//      1  A     L  S    I  G
//      2  Y  A     H  R
//      3  P        I
//  When i = 0, 3, the interval is (num_rows - 1) * 2.
//  Whe 0 < i < 3, the interval is (num_rows - 1) * 2 AND (num_rows - 1) * 2 - 2 * i
fn convert(s: String, num_rows: i32) -> String {
    let mut res = "".to_string();
    let n = num_rows as usize;
    if n == 1 {
        return s;
    }
    let m = s.len();
    let mut v: Vec<String> = vec!["".to_string(); n];
    let mut row = 0;
    let mut direction = true;
    for j in 0..m {
        v[row] += &s[j..=j];
        if row == 0 {
            direction = true;
            row += 1;
        } else if row == n - 1 {
            direction = false;
            row -= 1;
        } else {
            if direction {
                row += 1;
            } else {
                row -= 1;
            }
        }
    }
    for t in v {
        res += &t;
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::convert;

    #[test]
    fn test_1() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 3;
        assert_eq!(convert(s, num_rows), "PAHNAPLSIIGYIR".to_string());
    }

    #[test]
    fn test_2() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 4;
        assert_eq!(convert(s, num_rows), "PINALSIGYAHRPI".to_string());
    }

    #[test]
    fn test_3() {
        let s = "A".to_string();
        let num_rows = 1;
        assert_eq!(convert(s, num_rows), "A".to_string());
    }
}
