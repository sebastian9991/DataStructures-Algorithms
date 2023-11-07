public class Solution {
    public int removeDuplicates(int[] nums) {
        
	    int k = 0; 
	    int j = 1; 
	    //First loop to find deletion: 
	    for(int i = 0; i < nums.length; i++) {
		    if(nums[i] == nums[j]){
			    //Second loop to shift elements in the array
			    for(int l = j; l < nums.length - 1; l ++) {
				    nums[l] = nums[l + 1];
			    }
			    j++;
		    }
	    }
	    return k;
    }
}
