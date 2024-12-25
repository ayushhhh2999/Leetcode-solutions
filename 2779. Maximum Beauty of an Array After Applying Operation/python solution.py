class Solution:
    def maximumBeauty(self, nums: List[int], k: int) -> int:
        nums.sort()
        max_val = 0
        i = 0
        sum_val = 0
        for j in range(len(nums)):
           
            if nums[j] - nums[i] > 2*k:
                wind_size -= 1
                i += 1
            wind_size = j - i + 1
            max_val = max(max_val,wind_size)    
        return max_val
            

        