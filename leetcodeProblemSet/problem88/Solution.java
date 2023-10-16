class Solution {
    public void merge(int[] nums1, int m, int[] nums2, int n) {
        
        int p_1 = m - 1; 
        int p_2 = n - 1; 
        int p_m = m + n - 1; 

        while(p_2 >= 0 ) {
            if(p_1 >= 0 && nums1[p_1] > nums2[p_2]) {
                nums1[p_m] = nums1[p_1];
                p_m--; 
                p_1--;
            } else {
                nums1[p_m] = nums2[p_2];
                p_m--; 
                p_2--;
            }
        }
        
    }
}
