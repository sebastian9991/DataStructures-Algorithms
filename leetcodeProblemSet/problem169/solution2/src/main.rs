struct Solution {}
//This solution heavily relies on the requirment that there is a majority element (n/2) within the
//list
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut element = nums[0];

        for value in nums {
            if count == 0 {
                element = value;
            }

            if value == element {
                count += 1;
            } else {
                count -= 1;
            }
        }

        element
    }
}
fn main() {
    let nums = vec![3, 2, 2];
    let nums2 = vec![2, 2, 1, 1, 1, 2, 2];
    println!("{}", Solution::majority_element(nums));
    println!("{}", Solution::majority_element(nums2));
}
