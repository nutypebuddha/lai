fn main() {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=ATHENA_BUILD_TIME={}", timestamp);

    // Re-run if any formulas change
    println!("cargo:rerun-if-changed=formulas/");
}
