struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut prev = 0;
        for i in 1..nums.len() {
            if nums[prev] != nums[i] {
                prev += 1;
                nums[prev] = nums[i];
            }
        }
        return (prev + 1) as i32;
    }
}
fn main() {
    //Test here:
    let mut _nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    Solution::remove_duplicates(&mut _nums);
    println!("{:?}", _nums)
}
