public class Solution {
    public int removeElement(int[] nums, int val) {
        int [] nums_new = new int[nums.length]; 
        int k = 0; 

        for(int i = 0; i < nums.length; i ++) {

            if(nums[i] != val) {

                nums_new[i] = nums[i];
                k++;
                
            } else {

                nums_new[i] = -1; 
            }

            }

        //Reallocate the nums array to be nums_new 
        //
        int length = nums.length - 1;
        for(int i = 0; i < nums.length; i ++) {
            
            if (nums_new[i] == -1) {
                
                nums[length - i] = nums_new[i];
                
            } else {  
                nums[i] = nums_new[i];
            }
        }
        


       return k; 
    }


}
