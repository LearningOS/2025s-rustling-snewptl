//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

    // std::env::set_var("TEST_FOO", timestamp.to_string());
    println!("cargo:rustc-env=TEST_FOO={}", timestamp.to_string());
    println!("cargo:rustc-cfg=feature=\"pass\"");
}