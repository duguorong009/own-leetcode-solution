fn main() {
    println!("Hello, world!");
}
fn valid_palindrome(s: String) -> bool {
    let v: &str = s.as_str();
    if let Some(s) = is_palidrome(v) {
        let sl: &str = &s[1..];
        let sr: &str = &s[..s.len() - 1];
        is_palidrome(sl).is_none() || is_palidrome(sr).is_none()
    } else {
        true
    }
}

fn is_palidrome(v: &str) -> Option<&str> {
    let n = v.len();
    if n <= 1 {
        return None;
    }
    if v.chars().next() != v.chars().last() {
        Some(v)
    } else {
        is_palidrome(&v[1..n - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "aba".to_string();
        assert!(valid_palindrome(s));
    }

    #[test]
    fn test_2() {
        let s = "abca".to_string();
        assert!(valid_palindrome(s));
    }

    #[test]
    fn test_3() {
        let s = "abc".to_string();
        assert!(!valid_palindrome(s));
    }

    #[test]
    fn test_4() {
        let s = "rglqqcokqpyxyqvlackvrglaczrsusrwanzhcigjbplnkdnaqmnigkxercheohqdctnysrpjrcuragccvqlhwsoulhkwdevxkrduhewfphsnqstqjtkdptjtduzeqmwfwwgceaazxdkqsruypztrarqtbfuwznkadbeutvsavnqvbroftsgigkmzhlfvxzwrjseywkacwldlweeffsrgabrwqsxczbqkhfdbehvxvrypjtohhncjcvodaaaadovcjcnhhotjpyrvxvhebdfhkqbzcxsqwrbagrsffeewldlwcakwyesjrwzxvflhzmkgigstforbvqnvasvtuebdaknzwufbtqrartzpyursqkdsxzaaecgwwfwmqezudtjtpdktjqtsqnshpfwehudrkxvedwkhluoswhlqvccgarucrjprsyntcdqhoehcrexkginmqandknlpbjgichznsawrsusrzcalgrvkcalvqyxypqkocqqlgr".to_string();
        assert!(!valid_palindrome(s));
    }
}
