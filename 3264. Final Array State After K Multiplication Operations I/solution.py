class Solution:
    def getFinalState(self, nums: List[int], k: int, multiplier: int) -> List[int]:
        heap = []
        for index,val in enumerate(nums):
            heapq.heappush(heap,(val,index))
        for _ in range(k):
                val , index = heapq.heappop(heap)
                val *= multiplier 
                heapq.heappush(heap,(val,index))
        while heap:
            val,index = heapq.heappop(heap)
            nums[index] = val
        return nums            
        