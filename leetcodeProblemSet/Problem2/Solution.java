/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode() {}
 *     ListNode(int val) { this.val = val; }
 *     ListNode(int val, ListNode next) { this.val = val; this.next = next; }
 * }
 */
class Solution {
    public ListNode addTwoNumbers(ListNode l1, ListNode l2) {

       return addTwoNumbersCarry(l1, l2, false); 
    }

public ListNode addTwoNumbersCarry(ListNode l1, ListNode l2, boolean carry) {

        ListNode node;
        //Special case of both null with carry; 
        if(l1 == null && l2 == null && carry == true) {
            node = new ListNode(1);
            node.next = null; 
            return node; 
        }
        if(l1 != null || l2 != null) {
            //Create the val of a null node
            int val_1 = 0; 
            int val_2 = 0; 
            ListNode next_1 = null; 
            ListNode next_2 = null;
            if(l1 != null) {
                val_1 = l1.val; 
                //Create the next_1
                if(l1.next != null) {
                    next_1 = l1.next; 
                }
            }

            if(l2 != null) {
                val_2 = l2.val; 
                //Create the next_2 
                if(l2.next != null) {
                    next_2 = l2.next; 
                }
            }
            //Check the flag carry 
            int sum; 
            if(carry == true) {
             sum = val_1 + val_2 + 1; 
            } else{
             sum = val_1 + val_2; 
            }

            ListNode next; 
            if(sum >= 10) {
                //We deal with the carry that will always be one 
                int remainder = sum % 10;
                //Create the node: 
                node = new ListNode(remainder);
                //We must carry it here
                 next = addTwoNumbersCarry(next_1, next_2, true);
            } else {
                 node = new ListNode(sum);
                 next = addTwoNumbersCarry(next_1, next_2, false);  
            }
            node.next = next;
        } else {
        node = null; 
    }
    
    return node; 
    }
}
