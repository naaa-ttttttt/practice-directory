use std::collections::BinaryHeap;
pub fn k_closest(x: i32, k: i32, values: Vec<i32>) -> Vec<i32> {
    let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();

    for num in values {
        let distance = num.abs_diff(x);
        heap.push((distance.try_into().unwrap(), num));
    }

    if heap.len() > k.try_into().unwrap() {
        heap.pop();
    }

    let mut results: Vec<i32> = Vec::new();

    for closest in heap {
        let (_dist, val) = closest;

        results.push(val);
    }


    results.sort();

    results
}
