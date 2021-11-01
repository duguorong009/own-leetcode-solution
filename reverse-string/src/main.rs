fn main() {
    let mut s: Vec<&str> = vec!["o", "k"];
    reverse_string(&mut s);
    print!("{:?}", s);
}

fn reverse_string(s: &mut Vec<&str>) {
    if s.len() == 0 || s.len() == 1 {
        return;
    }
    let mut l = 0;
    let mut r = s.len() - 1;
    while l <= r {
        let temp = s[l];
        s[l] = s[r];
        s[r] = temp;
        l += 1;
        r -= 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::reverse_string;

    #[test]
    fn test_1() {
        let mut s: Vec<&str> = vec!["h", "e", "l", "l", "o"];
        reverse_string(&mut s);
        assert_eq!(s, vec!["o", "l", "l", "e", "h"]);
    }

    #[test]
    fn test_2() {
        let mut s: Vec<&str> = vec!["H", "a", "n", "n", "a", "h"];
        reverse_string(&mut s);
        assert_eq!(s, vec!["h", "a", "n", "n", "a", "H"]);
    }

    #[test]
    fn test_3() {
        let mut s: Vec<&str> = vec!["H"];
        reverse_string(&mut s);
        assert_eq!(s, vec!["H"]);
    }

    #[test]
    fn test_4() {
        let mut s: Vec<&str> = vec![];
        reverse_string(&mut s);
        assert_eq!(s.len(), 0);
    }
}
