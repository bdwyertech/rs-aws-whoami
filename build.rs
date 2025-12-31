use std::env;

fn main() {
    if let Ok(version) = env::var("BUILD_VERSION") {
        println!("cargo:rustc-env=BUILD_VERSION={}", version);
    }

    built::write_built_file().expect("Failed to acquire build-time information")
}
