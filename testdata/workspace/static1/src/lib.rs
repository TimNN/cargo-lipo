pub extern "C" fn hello_from_static1() {
    println!("Calling Normal from Static 1!");
    normal::hello_from_normal();
    println!("Hello from Static 1!");
}
