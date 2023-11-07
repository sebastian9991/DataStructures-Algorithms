public class Test {

public static void main(String[] args) {
    Solution s = new Solution();

    int[] case_1 =  {3,2,2,3};
    
    int k = s.removeElement(case_1, 3); // Calls your implementation

    System.out.println(k);

    for(int i = 0; i < case_1.length; i++) {

        System.out.println(case_1[i]);
    }

}  


}
