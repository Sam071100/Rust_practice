pub fn run() {
    println!("Hello, from the print.rs file");

    println!("{}",1234);
    println!("{} is enjoying {}", "Shubham Samrat", "Summer of Bitcoin");
    println!("{0} is from {1} and {0} likes to {2}","Shubham Samrat", "Mars", "Code");
    println!("{name} likes to play {activity}", name = "Shubham Samrat", activity = "PC Games");

    //Placeholder Traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10,10,10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "Hello"));
}