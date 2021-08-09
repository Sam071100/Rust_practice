pub fn run() {

    // Primitive str = Immutable fixed length string somewhere in the memory
    // String = growable, heap allocated data structure - use when you need to modify or own string data

    let hello = "Welcome"; // Primitive str type
    let mut hello1 = String::from("Hellooo");

    hello1.push('w'); // Pushes char to the String
    hello1.push_str(" Shubham Samrat"); // Pushes string to string basically appending

    println!("IS Empty: {}", hello1.is_empty());
    println!("Contains 'World': {}", hello1.contains("World"));

    println!("Replace: {}", hello1.replace("Hellooow", "Shubham"));
    println!("{}", hello1);

    for word in hello1.split_whitespace() {
        println!("{}", word);
    }

    // Create a String with certain capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    s.push('c');

    println!("{}", s);

    // Asserting testing
    // assert_eq!(5, s.len()); -->fails
    assert_eq!(10, s.capacity());

    println!("{:?}", (hello, hello1.capacity(), hello.len()));
}