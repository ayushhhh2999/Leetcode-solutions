use std::collections::BinaryHeap;
impl Solution {
    pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let mut min_heap = BinaryHeap::new();
        for (idx,val) in nums.iter().enumerate(){
            let idx = idx as i32;
            min_heap.push((-val,-idx as i32));
        }
        for i in 0..k{
           let (mut val,idx) = min_heap.pop().unwrap_or((0,0));
           val *= multiplier;
           min_heap.push((val,idx));
           let idx = -idx as i32;
            nums[idx as usize] = -val;
        }
        
        nums
    }
}