pub extern "C" fn hello_from_static2() {
    println!("Calling Build Util from Static 2!");
    buildutil::hello_from_build_util();
    println!("Hello from Static 2!");
}
