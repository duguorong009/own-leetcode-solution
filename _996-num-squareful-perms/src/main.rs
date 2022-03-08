fn main() {
    println!("Hello, world!");
}

fn num_squareful_perms(nums: Vec<i32>) -> i32 {
    let mut cnt = 0;

    // TODO: Get all the possible perms of "nums".
    // let possible_perms: Vec<Vec<i32>> = get_possible_perms(nums);
    let possible_perms: Vec<Vec<i32>> = vec![];
    for perm in possible_perms {
       // TODO
       // If the "perm" is squareful, increase the count.
    }
    cnt
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 17, 8];
        assert_eq!(num_squareful_perms(nums), 2);
    }

    #[test]
    fn test_2() {
        let nums = vec![2, 2, 2];
        assert_eq!(num_squareful_perms(nums), 1);
    }
}