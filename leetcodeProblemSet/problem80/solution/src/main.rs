struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        //Case of len 2
        if nums.len() < 2 {
            return nums.len() as i32;
        }

        let mut prev = 2;

        for i in 2..nums.len() {
            if nums[prev - 2] != nums[i] {
                nums[prev] = nums[i];
                prev += 1;
            }
        }
        return prev as i32;
    }
}
fn main() {
    let mut nums = vec![1, 1, 1, 2, 2, 3];
    let mut nums2 = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
    Solution::remove_duplicates(&mut nums);
    Solution::remove_duplicates(&mut nums2);
    println!("{:?}", nums);
    println!("{:?}", nums2);
    println!("{}", nums2.len());
}
