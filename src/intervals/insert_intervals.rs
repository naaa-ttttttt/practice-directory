pub fn insert(mut intervals: Vec<Vec<i32>>, new_intervals: Vec<i32>) -> Vec<Vec<i32>> {
    let mut  meetings: Vec<Vec<i32>> = Vec::new();

    intervals.sort_by(|a, b| a[0].cmp(&b[0]));

    // so i check if the start time of the new_intervals is less than the end time of the current interval
    // and i also check if the end time of the new interval is less than the start time of the next interval
    // if it satisfies, we push the values to the vector, else, we merge values where they overlap.


    // so the correction to the algorithm has changed, we create pointers,
    //that is the starting point of our loop through our intervals, we use
    //a conditional loop to check, while our starting interval is our starting 
    //point is less than the length, and while the end time of the current 
    //interval[i] is less than the start time of the new interval[0], 
    //is shows that there are no overlappoing values and so it pushes it to the result. 
    //and then while the start time of the current inteval[0] is less or equal to 
    //the end time of the new interval[1], we reassign the first values of the new interval 
    //by finding the minimum value ofthe current interval start times and we reassign the end times by finding the max of the new interval and the current interval.

    meetings
}
