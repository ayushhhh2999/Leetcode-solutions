class Solution:
    def trap(self, height: List[int]) -> int:
        max1 = 0
        max2 = 0
        left = [0]*len(height)
        right = [0]*len(height)
        j = len(height)-1
        total_area = 0
        for i in range(len(height)):
            left[i] = max1
            right[j] = max2
            max1 = max(max1,height[i])
            max2 = max(max2,height[j])
            j -= 1
        for x in range(len(left)):
            min1 = min(left[x],right[x])
            area = min1 - height[x]
            if area < 0:
                area = 0
            total_area += area
        return total_area      