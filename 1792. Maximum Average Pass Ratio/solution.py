import heapq

class Solution:
    def gain(self,passed: int,total: int) -> float:
        return (passed + 1)/(total+1) - (passed)/(total)
    def maxAverageRatio(self, classes: List[List[int]], extraStudents: int) -> float:
        max_heap = []
        for passed,total in classes:
            heapq.heappush(max_heap,(-self.gain(passed,total),passed,total))
        for _ in range(extraStudents):
            _, passed,total = heapq.heappop(max_heap)
            passed += 1
            total += 1
            heapq.heappush(max_heap,(-self.gain(passed,total),passed,total))
        max_ratio = 0
        for  _,i,j in max_heap:
            max_ratio += i/j
        return max_ratio/len(classes)            