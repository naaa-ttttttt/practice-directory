pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
    // check if the array is the list of intervals is empty
    // if its empty, return true.
    // we sort the intervals in ascending order, 
    // we loop through the array,
    // if the end time in the first array is less than the start time in the 
    // next array, return false.
    if intervals.is_empty() {
        return true;
    }
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    true
}
