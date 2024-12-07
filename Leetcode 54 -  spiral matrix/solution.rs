enum Direction{
    Up,
    Down,
    Right,
    Left,
}
impl Solution {
pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let m:usize = matrix.len();
    let n:usize = matrix[0].len();
    let mut up:usize = 0;
    let mut down:usize = m;
    let mut left:i32 = -1;
    let mut right:usize = n;
    let mut i:usize = 0;
    let mut j:usize = 0;
    let mut direction:Direction = Direction::Right;
    let mut res:Vec<i32> = vec![];
    while res.len() < m*n{
        match direction 
        {
        Direction::Right =>{
            while j < right{
                res.push(matrix[i][j]);
                j += 1;
            }
            i += 1;
            j -= 1;
            right -= 1;
            direction = Direction::Down;
        }
        Direction::Down =>{
            while i < down{
                res.push(matrix[i][j]);
                i += 1;
            }
            i -= 1;
            j -= 1;
            down -= 1;
            direction = Direction::Left;
        }
        Direction::Left =>{
            while j as i32 > left{
                res.push(matrix[i][j]);
                j -= 1;
            }
            i -= 1;
            j += 1;
            left += 1;
            direction = Direction::Up;
        }
        Direction::Up =>{
            while i > up{
                res.push(matrix[i][j]);
                i -= 1;
            }
            i += 1;
            j += 1;
            up += 1;
            direction = Direction::Right;
        }
    } }res
}
}