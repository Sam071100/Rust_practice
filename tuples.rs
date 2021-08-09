pub fn run() {
    // tuples are group together values of different types
    // Max 12 elements in tuples

    let person: (&str, &str, i8) = ("Shubham", "Mars", 20);
    println!("{} is from {} and is {}", person.0, person.1, person.2);
    println!("{:?}", (person));
}