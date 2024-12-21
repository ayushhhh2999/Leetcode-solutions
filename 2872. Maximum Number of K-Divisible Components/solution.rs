use std::collections::HashMap;
impl Solution {
    fn dfs(adj: &HashMap<i32,Vec<i32>>,values: &Vec<i32>,k:i32,curr:&i32,parent:&i32,count: &mut i32) -> i32{
        let mut total_sum = values[*curr as usize];
        if let Some(vec) = adj.get(&curr){
        for nbr in vec.iter(){
            if nbr != parent{
                total_sum += Self::dfs(adj,values,k,nbr,curr,count);
            }
        }}
        total_sum %= k;
        if total_sum == 0{
            *count += 1;
        }
        return total_sum 
    }
    pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32 {
        let mut adj:HashMap<i32,Vec<i32>> = HashMap::new();
        for edge in edges{
            let u = edge[0];
            let v = edge[1];
            adj.entry(u).or_insert_with(Vec::new).push(v);
            adj.entry(v).or_insert_with(Vec::new).push(u);
        }
        let mut count = 0;
        Self::dfs(&adj,&values,k,&0,&-1,&mut count);
        count
    }
}