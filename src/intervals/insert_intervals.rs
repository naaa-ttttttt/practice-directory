pub fn insert(mut intervals: Vec<Vec<i32>>, new_intervals: Vec<i32>) -> Vec<Vec<i32>> {
    let mut  meetings: Vec<Vec<i32>> = Vec::new();

    intervals.sort_by(|a, b| a[0].cmp(&b[0]));

    for i in 0..intervals.len() {
        if new_intervals[0] < intervals[i][0] {
            meetings.push(intervals[i].clone());
            continue;
        } //else if new_intervals[0]{}
    }

    meetings
}
