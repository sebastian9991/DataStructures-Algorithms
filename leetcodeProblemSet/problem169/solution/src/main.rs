struct Solution {}
use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for i in nums {
            let count = map.entry(i).or_insert(0);
            *count += 1;
        }

        return *map.iter().max_by_key(|entry| entry.1).unwrap().0;
    }
}
fn main() {
    let nums = vec![3, 2, 3];
    let nums2 = vec![2, 2, 1, 1, 1, 2, 2];
    println!("{}", Solution::majority_element(nums));
    println!("{}", Solution::majority_element(nums2));
}
