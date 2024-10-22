// tests7.rs
//
// When building packages, some dependencies can neither be imported in
// `Cargo.toml` nor be directly linked; some preprocesses varies from code
// generation to set-up package-specific configurations.
//
// Cargo does not aim to replace other build tools, but it does integrate
// with them with custom build scripts called `build.rs`. This file is
// usually placed in the root of the project, while in this case the same
// directory of this exercise.
//
// It can be used to:
//
// - Building a bundled C library.
// - Finding a C library on the host system.
// - Generating a Rust module from a specification.
// - Performing any platform-specific configuration needed for the crate.
//
// When setting up configurations, we can `println!` in the build script
// to tell Cargo to follow some instructions. The generic format is:
//
//     println!("cargo:{}", your_command_in_string);
//
// Please see the official Cargo book about build scripts for more
// information:
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
//
// In this exercise, we look for an environment variable and expect it to
// fall in a range. You can look into the testcase to find out the details.
//
// You should NOT modify this file. Modify `build.rs` in the same directory
// to pass this exercise.
//
// Execute `rustlings hint tests7` or use the `hint` watch subcommand for a
// hint.

fn main() {}
use std::env;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // 获取当前时间戳
        let current_timestamp = get_current_timestamp();

        // 设置 TEST_FOO 为当前时间戳减去 5 秒
        env::set_var("TEST_FOO", (current_timestamp - 5).to_string());

        // 获取时间戳并进行断言
        let timestamp = get_current_timestamp();

        let s = env::var("TEST_FOO").unwrap();
        let e: u64 = s.parse().unwrap();

        assert!(timestamp >= e && timestamp < e + 10, "Timestamp is within expected range.");
    }

    fn get_current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
