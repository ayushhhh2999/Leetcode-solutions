# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
#using bfs
class Solution:
    def reverseOddLevels(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        queue = deque([root])
        temp = 0
        parity = True
        while queue:
            n = len(queue)
            temp = []
            for i in range(n):
                a = queue.popleft()
                if a.left:
                    temp.append(a.left)
                    queue.append(a.left)
                if a.right:
                    temp.append(a.right)
                    queue.append(a.right)
            if parity:  
                j = len(temp)-1  
                for i in range(len(temp)//2):
                    if temp[j] and temp[i]:
                        temp[j].val,temp[i].val = temp[i].val,temp[j].val
                    j -= 1
            parity = not parity    
        return root


    #using dfs
    class Solution:
    def reverseOddLevels(self, root: Optional[TreeNode]) -> Optional[TreeNode]:

        def dfs(left: Optional[TreeNode], right: Optional[TreeNode], level: int):
            if not left or not right:
                return 
            if level % 2 == 1:
                left.val,right.val = right.val,left.val
                
            dfs(left.left , right.right,level+1)
            dfs(left.right , right.left,level+1)  
        if root:
            dfs(root.left,root.right,1)
        
        return root    