use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("second_data.data")?; // Replace with your file path
    let reader = BufReader::new(file);

    let mut safe_count = 0;

    for line in reader.lines() {
        let line = line?;
        let levels: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if is_safe(&levels) || is_safe_with_dampener(&levels) {
            safe_count += 1;
        }
    }

    println!("{}", safe_count);
    Ok(())
}

// Function to check if a report is safe
fn is_safe(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return false; // At least two levels are needed to compare
    }

    let diffs: Vec<i32> = levels.windows(2).map(|pair| pair[1] - pair[0]).collect();

    let all_increasing = diffs.iter().all(|&d| d > 0 && d <= 3);
    let all_decreasing = diffs.iter().all(|&d| d < 0 && d.abs() <= 3);

    all_increasing || all_decreasing
}

// Function to check if a report is safe after removing one level
fn is_safe_with_dampener(levels: &[i32]) -> bool {
    for i in 0..levels.len() {
        // Create a new vector excluding the `i`th level
        let mut modified_levels = levels.to_vec();
        modified_levels.remove(i);

        // Check if the modified report is safe
        if is_safe(&modified_levels) {
            return true;
        }
    }
    false
}
