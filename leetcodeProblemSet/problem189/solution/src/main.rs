struct Solution {}
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        for _ in 0..k {
            let save = nums[nums.len() - 1];
            for j in (1..nums.len()).rev() {
                nums[j] = nums[j - 1];
            }
            nums[0] = save;
        }
    }
}
fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    let mut nums2 = vec![-1, -100, 3, 99];
    Solution::rotate(&mut nums, 3);
    Solution::rotate(&mut nums2, 2);
    println!("{:?}", nums);
    println!("{:?}", nums2);
}
