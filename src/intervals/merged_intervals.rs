pub fn merge_intervals(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

    let output: Vec<Vec<i32>>  = Vec::new();

    intervals.sort_by(|a, b| a[0].cmp(&b[0]));

    for i in 1..intervals.len() {
        if intervals[i][0] >= intervals[i - 1][1] {
            intervals[i][0] = intervals[i][0].min(intervals[i - 1][0]);
            intervals[i][1] = intervals[i][1].max(intervals[i - 1][1]);
        }
    }
    output
} 
