class Solution:
    def trivialcase(self,heights:List[int],alice:int,bob:int) -> bool:
        if alice < bob and heights[alice] < heights[bob]:
            return True
        elif alice == bob:
            return True
        else:
            return False        
    def leftmostBuildingQueries(self, heights: List[int], queries: List[List[int]]) -> List[int]:
        res = [-1]*len(queries)
        track = defaultdict(list)
        for idx,(alice,bob) in enumerate(queries):
            if alice > bob:
                    alice , bob = bob ,alice
            if self.trivialcase(heights,alice,bob):
                res[idx] = bob    
            else:
                track[bob].append((max(heights[alice],heights[bob]),idx))

        min_heap = []        
        for idx,height in enumerate(heights):
            while min_heap and min_heap[0][0] < height:
                _, q_idx = heapq.heappop(min_heap)
                res[q_idx] = idx 
               
            for (h,q) in track[idx]:    
                heapq.heappush(min_heap,(h,q))
        return res            

        