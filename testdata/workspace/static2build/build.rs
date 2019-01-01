fn main() {
    println!("Calling Build Util from Static 2 build.rs!");
    buildutil::hello_from_build_util();
    println!("Hello from Static 2 build.rs!");
}
