pub fn internal_hello() {
    println!("Internal Hello from Static 3!");
}

pub extern "C" fn hello_from_static3() {
    println!("Calling Internal from Static 3 lib!");
    internal_hello();
    println!("Internal Hello from Static 3 lib!");
}
