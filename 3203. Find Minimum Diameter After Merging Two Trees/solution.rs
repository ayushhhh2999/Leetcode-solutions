use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    fn diameter(adj: HashMap<usize, Vec<usize>>, n: usize) -> i32 {
        let mut q = VecDeque::new();
        q.push_back(0);
        let mut last = 0;
        let mut visited = vec![false; n];
        visited[0] = true;
        while !q.is_empty() {
            let size = q.len();
            for _ in 0..size {
                if let Some(node) = q.pop_front() {
                    last = node;
                    if let Some(neighbors) = adj.get(&node) {
                        for &ele in neighbors {
                            if !visited[ele] {
                                visited[ele] = true;
                                q.push_back(ele);
                            }
                        }
                    }
                }
            }
        }
        q.push_back(last);
        visited = vec![false; n];
        visited[last] = true;
        let mut hops = 0;

        
        while !q.is_empty() {
            let size = q.len();
            for _ in 0..size {
                if let Some(node) = q.pop_front() {
                    if let Some(neighbors) = adj.get(&node) {
                        for &ele in neighbors {
                            if !visited[ele] {
                                visited[ele] = true;
                                q.push_back(ele);
                            }
                        }
                    }
                }
            }
            hops += 1;
        }

        hops - 1 
    }

    fn find_diameter(edges: Vec<Vec<i32>>) -> i32 {
        if edges.len() == 0{
            return 0
        }
        let mut nodes = HashSet::new();
        let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();

       
        for edge in edges.iter() {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adj.entry(u).or_insert_with(Vec::new).push(v);
            adj.entry(v).or_insert_with(Vec::new).push(u);
            nodes.insert(u);
            nodes.insert(v);
        }

        Self::diameter(adj, nodes.len())
    }

    pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
        let r1 = Self::find_diameter(edges1);
        let r2 = Self::find_diameter(edges2);
        let radii_sum = (r1 + 1) / 2 + (r2 + 1) / 2 + 1;

        std::cmp::max(radii_sum, std::cmp::max(r1, r2))
    }
}
