enum Dir{
    Up,
    Down,
    Left,
    Right,
}
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![vec![0;n as usize] ; n as usize];
        let mut up:usize = 0;
        let mut down:usize = n as usize;
        let mut right:usize = n as usize;
        let mut left:i32 = -1;
        let mut dir:Dir = Dir::Right;
        let mut i:usize = 0;
        let mut j:usize = 0;
        let mut x = 1;
        while x <= n*n{
            match dir{
                Dir::Right =>{
                    while j < right{
                        res[i][j] = x;
                        x += 1;
                        j += 1;
                    }
                    i += 1;
                    j -= 1;
                    right -= 1;
                    dir = Dir::Down;
                }
                Dir::Down =>{
                    while i < down{
                        res[i][j] = x;
                        x += 1;
                        i += 1;
                    }
                    i -= 1;
                    j -= 1;
                    down -= 1;
                    dir = Dir::Left;
                }
                Dir::Left =>{
                    while j as i32 > left{
                        res[i][j] = x;
                        x += 1;
                        j -= 1;
                    }
                    i -= 1;
                    j += 1;
                    left += 1;
                    dir = Dir::Up;
                }
                Dir::Up =>{
                    while i > up{
                        res[i][j] = x;
                        x += 1;
                        i -= 1;
                    }
                    i += 1;
                    j += 1;
                    up += 1;
                    dir = Dir::Right;
                }
            } 
        }res   
        
    }
}