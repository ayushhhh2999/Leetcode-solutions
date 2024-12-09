class Solution:
    def parity(self,x:int):
        if x%2 == 0:
            return True
        return False   
    def isArraySpecial(self, nums: List[int], queries: List[List[int]]) -> List[bool]:
        count = 0
        prefix_array = [0]
        for i  in range(1, len(nums)):
            if self.parity(nums[i-1]) == self.parity(nums[i]):
                count += 1
            prefix_array.append(count)
        res = []
        for start , end in queries:
            if prefix_array[end] -  prefix_array[start]  == 0:
                res.append(True)
            else:
                res.append(False)    
        return res


            

        