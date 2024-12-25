use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn findswaps(arr: &mut Vec<i32>) -> i32 {
        let mut idx_map: HashMap<i32, usize> = HashMap::new();
        for (idx, &val) in arr.iter().enumerate() {
            idx_map.insert(val, idx);
        }

        arr.sort();

        let mut seen: HashSet<usize> = HashSet::new();
        let mut swaps = 0;

        for &val in idx_map.keys() {
            let mut cycles = 0;
            let mut x = val;

            while let Some(&idx_new) = idx_map.get(&x) {
                if seen.contains(&idx_new) {
                    break;
                }
                seen.insert(idx_new);
                x = arr[idx_new];
                cycles += 1;
            }

            if cycles > 1 {
                swaps += cycles - 1;
            }
        }

        swaps
    }

    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        queue.push_back(root.clone());
        let mut total_operations = 0;

        while !queue.is_empty() {
            let n = queue.len();
            let mut temp = vec![];

            for _ in 0..n {
                if let Some(Some(node)) = queue.pop_front() {
                    let node = node.borrow();

                    temp.push(node.val);

                    if let Some(left) = node.left.clone() {
                        queue.push_back(Some(left));
                    }
                    if let Some(right) = node.right.clone() {
                        queue.push_back(Some(right));
                    }
                }
            }

            total_operations += Self::findswaps(&mut temp);
        }

        total_operations
    }
}
