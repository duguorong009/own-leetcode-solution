fn main() {
    println!("Hello, world!");
}

fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut slice = vec![];
    for x in nums {
        println!("{:?}", slice);
        if slice.len() < 2 {
            slice.push(x);
        } else {
            let y = slice.pop().unwrap();
            let z = slice.pop().unwrap();
            if y - z == x - y {
                slice.push(z);
                slice.push(y);
                slice.push(x);
                res += slice.len() - 2;
            } else {
                slice.clear();
                slice.push(y);
                slice.push(x);
            }
        }
    }
    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(number_of_arithmetic_slices(nums), 3);
    }

    #[test]
    fn test_2() {
        let nums = vec![1];
        assert_eq!(number_of_arithmetic_slices(nums), 0);
    }

    #[test]
    fn test_3() {
        let nums = vec![3, -1, -5, -9];
        assert_eq!(number_of_arithmetic_slices(nums), 3);
    }
}