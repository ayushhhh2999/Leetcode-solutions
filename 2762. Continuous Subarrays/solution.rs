impl Solution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut left = 0;
        let mut right = 0;
        let mut count = 0;
        let mut min_range = i64::MAX;
        let mut max_range = i64::MIN;
        while right < n{
            max_range = std::cmp::max(max_range,nums[right] as i64);
            min_range = std::cmp::min(min_range,nums[right] as i64);
            if max_range - min_range > 2{
                let wind_size = (right - left) as i64;
                count += ((wind_size)*(wind_size+1))/2;
                min_range = nums[right] as i64;
                max_range = nums[right] as i64;
                left = right;
                while left > 0 && (nums[right]-nums[left-1]).abs() <= 2{
                    left -= 1;
                    max_range = std::cmp::max(max_range,nums[left] as i64);
                    min_range = std::cmp::min(min_range,nums[left] as i64);
                    
                }
                if left < right{
                    let win_size = (right - left) as i64;
                    count -= ((win_size)*(win_size+1))/2;
                }

            } right += 1;

        }
        let wind_size = (right - left) as i64;
        return count + ((wind_size)*(wind_size+1))/2
        
    }
}