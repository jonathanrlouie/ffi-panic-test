fn main() {
    println!("cargo::rustc-link-search=../panic-lib/target/debug");
    println!("cargo::rustc-link-lib=static=panic_lib");
}
