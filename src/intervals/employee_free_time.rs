pub fn employee_free_time(schedules: Vec<Vec<Vec<i32>>>) -> Vec<Vec<i32>>{
    let mut flattened_intervals: Vec<Vec<i32>> = Vec::new();

    for sets in schedules {
        for intervals in sets {
            flattened_intervals.push(intervals);
        }
    }

    let mut free_time = Vec::new();

    free_time
}
