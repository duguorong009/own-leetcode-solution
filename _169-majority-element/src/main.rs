fn main() {
    println!("Hello, world!");
}

fn majority_element(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut num: Option<i32> = None;
    for x in nums {
        if let Some(y) = num {
            if x == y {
                count += 1;
            } else {
                count -= 1;
                if count == 0 {
                    num = None;
                }
            }
        } else {
            num = Some(x);
            count += 1;
        }
    }
    num.unwrap()
}
