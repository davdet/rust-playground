use std::collections::HashMap;

pub fn vectors() {
    let mut numbers: Vec<i32> = vec![
        12, 15, 6, 44, 6, 32, 77, 77, 77, 77, 77, 94, 6, 2, 67, 0, 25, 14, 84, 81, 59, 31, 7, 9,
    ];

    println!("{:?}", numbers);

    // calculating and displaying the mean
    let mut sum = 0;
    for num in &numbers {
        sum += *num;
    }
    let mean = sum as f64 / numbers.len() as f64;
    println!("Mean: {}", mean);

    // calculating and displaying the median
    let sorted_numbers = &mut numbers;
    sorted_numbers.sort();
    let median_index = sorted_numbers.len() / 2;
    println!("Median: {}", sorted_numbers[median_index]);

    // calculating and displaying the mode
    let mut numbers_count = HashMap::new();
    for num in &numbers {
        let count = numbers_count.entry(num).or_insert(0);
        *count += 1;
    }
    let mut mode = 0;
    let mut prev_value = 0;
    for (key, value) in &numbers_count {
        if *value > prev_value {
            prev_value = *value;
            mode = **key;
        }
    }
    println!("Mode: {}", mode);
}
