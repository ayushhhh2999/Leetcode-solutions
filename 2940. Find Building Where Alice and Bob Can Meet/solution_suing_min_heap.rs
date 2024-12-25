use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;
use std::mem::swap;

impl Solution {
    pub fn trivial_case(heights: &Vec<i32>, alice: usize, bob: usize) -> bool {
        if alice < bob && heights[alice] < heights[bob] {
            true
        } else if alice == bob {
            true
        } else {
            false
        }
    }

    pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut track: HashMap<usize, Vec<(i32, usize)>> = HashMap::new();
        let mut res = vec![-1; queries.len()];

        for (idx, que) in queries.iter().enumerate() {
            let mut alice = que[0] as usize;
            let mut bob = que[1] as usize;
            if alice > bob {
                swap(&mut alice, &mut bob);
            }

            if Self::trivial_case(&heights, alice, bob) {
                res[idx] = bob as i32;
            } else {
                let max_height = heights[alice].max(heights[bob]);
                track.entry(bob).or_insert_with(Vec::new).push((max_height, idx));
            }
        }

        let mut min_heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
        for (idx, height) in heights.into_iter().enumerate() {
            while let Some(Reverse((max_height, r_idx))) = min_heap.pop() {
                if height > max_height {
                    res[r_idx] = idx as i32;
                } else {
                    min_heap.push(Reverse((max_height, r_idx)));
                    break;
                }
            }

            if let Some(max_que) = track.get(&idx) {
                for &(new_height, new_idx) in max_que {
                    min_heap.push(Reverse((new_height, new_idx)));
                }
            }
        }

        res
    }
}
