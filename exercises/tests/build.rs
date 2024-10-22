//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

// build.rs

fn main() {
    // 在 tests7 中，我们应该设置一个名为 `TEST_FOO` 的环境变量。
    // 在标准输出中打印出来，让 Cargo 处理它。
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Failed to get current timestamp")
        .as_secs();
    
    // 设置 TEST_FOO 环境变量
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 在 tests8 中，我们应该启用 "pass" 特性以使测试用例提前返回。
    // 填写命令以告知 Cargo 启用该特性。
    println!("cargo:rustc-cfg=feature=\"pass\"");
}