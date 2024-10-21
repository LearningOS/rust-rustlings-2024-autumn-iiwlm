// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // 不再调用 unwrap，因为 is_none 已经确认为 None
        println!("my_option is None");
    }

    let my_arr = &[-1, -2, -3, -4, -5, -6]; // 修正数组定义
    println!("My array! Here it is: {:?}", my_arr);

    let my_empty_vec: Vec<i32> = vec![]; // 创建一个空向量
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}