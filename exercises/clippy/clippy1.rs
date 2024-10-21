// clippy1.rs
//
// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy
// warnings check clippy's suggestions from the output to solve the exercise.
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.
fn main() {
    // 使用标准库提供的 PI 常量
    let pi = std::f32::consts::PI;
    let radius = 5.0f32;

    let area = pi * radius.powf(2.0);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    );
}