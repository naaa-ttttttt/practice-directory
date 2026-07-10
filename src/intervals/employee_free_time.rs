pub fn employee_free_time(schedules: Vec<Vec<Vec<i32>>>) -> Vec<Vec<i32>>{
    let mut flattened_intervals: Vec<Vec<i32>> = Vec::new();

    for sets in schedules {
        for intervals in sets {
            flattened_intervals.push(intervals);
        }
    }

    let mut merged: Vec<Vec<i32>> = Vec::new();

    for overlaps in 1..flattened_intervals.len() {
        if merged.is_empty() || flattened_intervals[overlaps][0] < merged.last().unwrap()[1] {
            merged.push(flattened_intervals[overlaps].clone());
        }
    }

    let mut free_time = Vec::new();

    free_time
}
