use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd)]
struct FloatWrapper(f64);

impl Eq for FloatWrapper {}

impl Ord for FloatWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn gain(passed: i32, total: i32) -> f64 {
        let passed = passed as f64;
        let total = total as f64;
        (passed + 1.0) / (total + 1.0) - passed / total
    }

    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut max_heap: BinaryHeap<(FloatWrapper, i32, i32)> = BinaryHeap::new();

        for class in classes.iter() {
            let passed = class[0];
            let total = class[1];
            max_heap.push((FloatWrapper(Self::gain(passed, total)), passed, total));
        }

        for _ in 0..extra_students {
            let (_, mut passed, mut total) = max_heap.pop().unwrap();
            passed += 1;
            total += 1;
            max_heap.push((FloatWrapper(Self::gain(passed, total)), passed, total));
        }

        let mut max_ratio: f64 = 0.0;
        while let Some((_, passed, total)) = max_heap.pop() {
            max_ratio += passed as f64 / total as f64;
        }

        max_ratio / classes.len() as f64
    }
}
