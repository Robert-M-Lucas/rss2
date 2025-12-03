fn main() {
    println!(
        "cargo:rustc-env=TARGET={}",
        build_target::target().triple
    );
}
