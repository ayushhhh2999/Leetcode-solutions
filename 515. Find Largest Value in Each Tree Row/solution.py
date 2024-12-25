#using BFS
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def largestValues(self, root: Optional[TreeNode]) -> List[int]:
        if not root:
            return []
        queue = deque([root])
        res = []
        while queue:
            n = len(queue)
            max_node = -98765432199877
            for i in range(n):
                node = queue.popleft()
                if node:
                    max_node = max(node.val,max_node)
                if node and node.left:
                    queue.append(node.left)
                if node and node.right:
                    queue.append(node.right)
            res.append(max_node)            
        return res 
        
#using DFS
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def largestValues(self, root: Optional[TreeNode]) -> List[int]:
        def find(root,res,level):
            if not root:
                return []
            if level == len(res):
                res.append(root.val)    
            else:
                res[level] = max(res[level],root.val)
            find(root.left,res,level+1)
            find(root.right,res,level+1)
        res = []  
        find(root,res,0)  
        return res  
                
