pub fn non_overlapping_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {

    if intervals.is_empty() {
        return 0;
    }
    let mut count = 0;

    intervals.sort_by(|a, b| a[1].cmp(&b[1]));

    for i in 1..intervals.len() {
        if intervals[i][0] < intervals[i - 1][1] {
            count += 1;
        }
    }


    count
}
