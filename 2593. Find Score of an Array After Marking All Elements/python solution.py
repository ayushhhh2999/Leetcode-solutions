class Solution:
    def findScore(self, nums: List[int]) -> int:
        arr = [(val,idx) for idx,val in enumerate(nums)]
        heapq.heapify(arr)
        count = 0
        for i in range(len(nums)):
            val,idx = heapq.heappop(arr)
            if nums[idx] < 0:
                continue
            elif val > 0:
                count += val
                nums[idx] = -nums[idx]
                if idx-1 >=0 and nums[idx-1] > 0:
                    nums[idx-1] = -nums[idx-1]
                if idx+1 < len(nums) and nums[idx +1] > 0:
                    nums[idx+1] = -nums[idx+1]       
        return count            

        
        