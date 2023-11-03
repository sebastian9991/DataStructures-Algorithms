class Solution {
    public int removeElement(int[] nums, int val) {
        int i = 0;
        //Simply switch positions if the next jth element is not equal to val; there by "pushing" the val elements 
        //to the end of the array
        for (int j = 0; j < nums.length; j++) {
            if (nums[j] != val) {
                int temp = nums[i];
                nums[i] = nums[j];
                nums[j] = temp;
                i++;
            }
        }
        return i;
    }
}
