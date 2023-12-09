struct Solution {}
impl Solution {
    //Thanks to: philipdaquin
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut last_index = nums.len() - 1;
        for i in (0..nums.len()).rev() {
            if i + nums[i] as usize >= last_index {
                last_index = i
            }
        }
        last_index == 0
    }
}
fn main() {}
