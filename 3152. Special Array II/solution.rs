impl Solution {
    pub fn parity(i : i32) -> bool{
        if i%2 == 0{
            return true
        }
        false
    }
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut prefix_sum = vec![0];
        let mut count = 0;
        for i in 1..nums.len(){
            if Self::parity(nums[i]) == Self::parity(nums[i-1]){
                count += 1;
            }
            prefix_sum.push(count);
        }
        let mut res = vec![];
        for num in queries{
            let start:usize = num[0] as usize;
            let end:usize = num[1] as usize;
            res.push(prefix_sum[end]-prefix_sum[start]==0);
        } 
        res
    }
}