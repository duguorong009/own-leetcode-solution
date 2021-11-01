use std::{collections::HashMap, vec};

fn main() {
    let nums: Vec<i32> = vec![2, 7, 11, 14];
    let target: i32 = 9;

    let mut supplement_num_hash_table: HashMap<i32, usize> = HashMap::new();
    for (id, &num) in nums.iter().enumerate() {
        supplement_num_hash_table.entry(num).or_insert(id);
    }

    for (id1, &num) in nums.iter().enumerate() {
        let result = supplement_num_hash_table.get(&(target - num));
        match result {
            None => continue,
            Some(&id2) => {
                println!("{:?}", vec![id1, id2]);
                break;
            }
        }
    }
}
