//using BFS
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut q:VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        if let Some(first) = root.as_ref(){
            q.push_back(first.clone());
        }
        let mut res = Vec::new();
        while !q.is_empty(){
            let n = q.len();
            let mut max_val = i32::MIN;
            for i in 0..n{
                if let Some(n) = q.pop_front(){
                    
                        let node = n.borrow();
                        max_val = std::cmp::max(node.val,max_val);
                        if let Some(left) = node.left.as_ref(){
                            q.push_back(left.clone());
                        }
                        if let Some(right) = node.right.as_ref(){
                            q.push_back(right.clone());
                        }
                    
                }
            }res.push(max_val);
        } res

    }
}
//using DFS
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, res:&mut Vec<i32>,level:usize){
        if root.as_ref() == None{
            return 
        }
       
            if let Some(n) = root{
            let node = n.borrow();
            if level == res.len(){
             res.push(node.val);
            }
            else{
                res[level] = std::cmp::max(node.val,res[level]);
            }
            Self::dfs(node.left.clone(),res,level+1);
            Self::dfs(node.right.clone(),res,level+1);
            
        }
    }
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut level = 0;
        let mut res = Vec::new();
        Self::dfs(root,&mut res,level);
        res
        
    }
}