pub fn employee_free_time(schedules: Vec<Vec<Vec<i32>>>) -> Vec<Vec<i32>>{
    let mut flattened_intervals: Vec<Vec<i32>> = Vec::new();

    for sets in schedules {
        for intervals in sets {
            flattened_intervals.push(intervals);
        }
    }

    flattened_intervals.sort_by_key(|x| x[0]);

    let mut merged: Vec<Vec<i32>> = Vec::new();

    for overlaps in flattened_intervals {
        if merged.is_empty() || overlaps[0] > merged.last().unwrap()[1] {
            merged.push(overlaps.clone());
        } else {
           let last_id = merged.len() - 1;
           merged[last_id][1] = merged[last_id][1].max(overlaps[1]);
        }
    }

    let mut free_time: Vec<Vec<i32>> = Vec::new();

    for i in 1..merged.len() {
        let start =  merged[i - 1][1];
        let end = merged[i][0];
        if start < end {
            free_time.push(vec![start, end]);
        }
    }
    free_time
}
