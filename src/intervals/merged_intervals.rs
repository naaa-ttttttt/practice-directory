pub fn merge_intervals(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // Handle edge case of empty input
        if intervals.is_empty() {
            return Vec::new();
        }

        // Sort intervals by start time to process overlaps sequentially
        intervals.sort_by_key(|interval| interval[0]);

        let mut merged: Vec<Vec<i32>> = Vec::new(); // Result vector to store merged intervals

        // Process each interval in sorted order
        for interval in intervals {
            // If no intervals in result OR current interval doesn't overlap with last merged
            if merged.is_empty() || interval[0] > merged.last().unwrap()[1] {
                merged.push(interval); // Add as new separate interval
            } else {
                // Current interval overlaps with last merged interval
                // Extend the end time of last merged interval to include current interval
                let last_idx = merged.len() - 1;
                merged[last_idx][1] = merged[last_idx][1].max(interval[1]);
            }
        }

        merged
}
