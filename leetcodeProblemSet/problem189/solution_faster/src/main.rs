struct Solution {}
impl Solution {
    pub fn reverse_vector(nums: &mut Vec<i32>, mut left: usize, mut right: usize) {
        while left < right {
            nums.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        if nums.len() == 1 || k == 0 || nums.len() as i32 == k {
            return;
        }
        //Notice we can achieve rotation by three reveres operation of the vector, allowing for
        //O(n) time!

        // set k as mod of nums length
        let k = k as usize % nums.len();

        Self::reverse_vector(nums, 0, nums.len() - 1);

        Self::reverse_vector(nums, 0, k - 1);

        Self::reverse_vector(nums, k, nums.len() - 1);
    }
}
fn main() {}
