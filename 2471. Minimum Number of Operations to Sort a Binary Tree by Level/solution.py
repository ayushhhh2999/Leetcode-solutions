# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def min_swaps(self,nums:List[int]) -> int:
        idx_map = {}
        for idx,val in enumerate(nums):
            idx_map[val] = idx
        nums.sort()
        seen = set()
        swaps1 = 0
        for vals in idx_map:
            if vals != nums[idx_map[vals]]:
                x = vals
                cycle = 0
                while idx_map[x] not in seen:    
                    new_idx = idx_map[x]
                    seen.add(idx_map[x])
                    x = nums[new_idx] 
                    cycle += 1
                swaps1 += cycle - 1 if cycle > 1 else 0
         
        return swaps1    

    def minimumOperations(self, root: Optional[TreeNode]) -> int:
        queue = deque([root])
        swaps = 0
        while queue:
            n = len(queue)
            temp = []
            for i in range(n):
                node = queue.popleft()
                if node.left:
                    queue.append(node.left)
                    temp.append(node.left.val)
                if node.right:
                    queue.append(node.right)
                    temp.append(node.right.val)    
            swaps += self.min_swaps(temp)  
        return swaps          
    

        