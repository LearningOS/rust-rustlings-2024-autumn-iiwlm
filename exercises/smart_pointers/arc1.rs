// arc1.rs
//
// In this exercise, we are given a Vec of u32 called "numbers" with values
// ranging from 0 to 99 -- [ 0, 1, 2, ..., 98, 99 ] We would like to use this
// set of numbers within 8 different threads simultaneously. Each thread is
// going to get the sum of every eighth value, with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, ...
// The second thread (offset 1), will sum 1, 9, 17, ...
// The third thread (offset 2), will sum 2, 10, 18, ...
// ...
// The eighth thread (offset 7), will sum 7, 15, 23, ...
//
// Because we are using threads, our values need to be thread-safe.  Therefore,
// we are using Arc.  We need to make a change in each of the two TODOs.
//
// Make this code compile by filling in a value for `shared_numbers` where the
// first TODO comment is, and create an initial binding for `child_numbers`
// where the second TODO comment is. Try not to create any copies of the
// `numbers` Vec!
//
// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.

use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<u32> = (0..100).collect();
    let shared_numbers = Arc::new(numbers); // 使用 Arc 包装 Vec

    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers); // 使用 Arc::clone 来共享数据
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().enumerate().filter_map(|(i, &n)| {
                if i % 8 == offset {
                    Some(n)
                } else {
                    None
                }
            }).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }

    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}