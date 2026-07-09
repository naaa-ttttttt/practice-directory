pub fn merge_intervals(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

    let output: Vec<Vec<i32>>  = Vec::new();

    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    output
} 
