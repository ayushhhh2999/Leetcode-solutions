from collections import deque, defaultdict
class Solution:
    def diameter(self, adj, n):
        visited = [False]*n
        queue = deque([0])
        visited[0] = True
        last = -1
        while queue:
            length = len(queue)
            for _ in range(length):
                last = queue.popleft()
                for ele in adj[last]:
                    if not visited[ele]:
                        visited[ele] = True
                        queue.append(ele)  
        visited = [False]*n
        queue = deque([last])
        visited[last] = True
        curr = -1 
        hops = 0
        while queue:
            length = len(queue)
            for _ in range(length):
                curr = queue.popleft()
                for ele in adj[curr]:
                    if not visited[ele]:
                        visited[ele] = True
                        queue.append(ele)  
            hops += 1
        return hops-1                

    def findDiameter(self, edges):
        if not edges:
            return 0
        nodes = set()
        adj = defaultdict(list)
        for edge in edges:
            adj[edge[0]].append(edge[1])  
            adj[edge[1]].append(edge[0])        
            nodes.add(edge[0])
            nodes.add(edge[1])
        return self.diameter(adj,len(nodes))
    
    def minimumDiameterAfterMerge(self, edges1, edges2):
        dia1 = self.findDiameter(edges1)
        dia2 = self.findDiameter(edges2)
        
        radius1 = (dia1 + 1) // 2
        radius2 = (dia2 + 1) // 2
        sum_radius = 1 + radius1 + radius2
        return max(sum_radius, max(dia1, dia2))