/*
https://leetcode.com/problems/add-two-numbers/

Solutions complexities
Time complexity: O(n1 + n2)
Space complexity: O(n1 + n2)
*/

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */

class Solution {
    /* 
    use a vector to collect values and later create a linked list
    This will save time used up in allocation of linked list in each loop
    */
public:
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        vector<int> vec;
        bool borrow = false;
        while(l1 != nullptr || l2 != nullptr)
       {
            unsigned int sum = 0;
            if(l1!=nullptr)
            {
                sum += l1->val; 
                l1 = l1->next;
            }
            if(l2!=nullptr)
            {
                sum += l2->val; 
                l2 = l2->next;
            }
            if (borrow) sum+=1;
            if (sum > 9) borrow = true;
            else borrow = false;
            vec.push_back(sum%10); 
       }
        
        if (borrow)
        {
            vec.push_back(1);
        }
        
        int index = 0;
        ListNode* head = new ListNode();
        ListNode* temp = head;
        
        auto first = vec.begin();
        temp->val = *first;
        temp->next = nullptr;
        
        for (auto iter = vec.begin() + 1; iter != vec.end(); ++iter)
        {
            temp->next = new ListNode();
            temp = temp->next;
            temp->val = *iter;
            temp->next = nullptr;
        }
        return head;
    }
};
