impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;

        let mut heap: BinaryHeap<Reverse<(i32, usize)>> = nums
            .iter()
            .enumerate()
            .map(|(idx, &val)| Reverse((val, idx)))
            .collect();
        
        let mut nums = nums; 
        let n = nums.len();
        let mut count = 0;

        for _ in 0..n {
            if let Some(Reverse((curr_val, curr_idx))) = heap.pop() {
                if nums[curr_idx] > 0 {
                    count += curr_val as i64;
                    nums[curr_idx] = -nums[curr_idx];
                    if curr_idx > 0 && nums[curr_idx - 1] > 0 {
                        nums[curr_idx - 1] = -nums[curr_idx - 1];
                    }
                    if curr_idx + 1 < nums.len() && nums[curr_idx + 1] > 0 {
                        nums[curr_idx + 1] = -nums[curr_idx + 1];
                    }
                }
            }
        }
        count
    }
}
