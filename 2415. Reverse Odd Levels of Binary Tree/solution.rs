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
use std::collections::VecDeque;
use core::mem::swap;
impl Solution {
    pub fn reverse_odd_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut queue:VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        queue.push_back(root.clone());
        let mut temp:Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
        let mut parity = true;
        while !queue.is_empty(){
            let range = queue.len();
            let mut temp = vec![];
            for _ in 0..range{
                   if let Some(n) = queue.pop_front(){

                    let mut node = n.as_ref().unwrap().borrow();
                    if let Some(left) = node.left.as_ref(){
                        temp.push(left.clone());
                        queue.push_back(Some(left.clone()));
                    } 
                    if let Some(right) = node.right.as_ref(){
                        temp.push(right.clone());
                        queue.push_back(Some(right.clone()));
                    } }}
                
             if parity == true{
                let mut right = temp.len()-1;
                for left in 0..temp.len()/2{
                    swap(&mut temp[left ].borrow_mut().val,
                         &mut temp[right].borrow_mut().val);
                    right -= 1;     
                }
               
            }  parity = !parity;
            
        } root



    }}
