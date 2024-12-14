class Solution:
    def continuousSubarrays(self, nums):
        n = len(nums)
        left = 0
        range_min = float('inf')
        range_max = float('-inf')
        count = 0
        right = 0
        while right < n:
            range_min = min(range_min,nums[right])
            range_max = max(range_max,nums[right])
            if range_max - range_min > 2:
                wind_size = right - left
                count += ((wind_size)*(wind_size + 1))//2
                left = right
                range_min = nums[right]
                range_max = nums[right]
                while left > 0 and abs(nums[right] - nums[left-1]) <= 2:
                    left -= 1
                    range_min = min(range_min,nums[left])
                    range_max = max(range_max,nums[left])
                if left < right: 
                    new_wind = right - left     
                    count -= ((new_wind)*(new_wind+1))//2
            right += 1
        count += ((right - left)*(right - left + 1))//2
        return count            


