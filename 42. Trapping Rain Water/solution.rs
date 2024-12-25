impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut max1 = 0;
        let mut max2 = 0;
        let n = height.len();
        let mut j = n-1;
        let mut left = vec![0;n];
        let mut right = vec![0;n];
        for i in 0..n{
            left[i] = max1;
            right[j] = max2;
            max1 = std::cmp::max(max1,height[i]);
            max2 = std::cmp::max(max2,height[j]);
            j -= 1;
        }
        let mut max_area = 0;
        for x in 0..n{
        let area = std::cmp::min(left[x],right[x]) - height[x];
        if area > 0{
            max_area += area;
        }   
        } max_area
    }
}