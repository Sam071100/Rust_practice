pub fn run() {
    let name = "Shubham Samrat";
    let mut age = 20;

    println!("My name is {} and my age is {}", name, age);

    age = 21;
    println!("My name is {} and my age is {}", name, age);

    const ID: i32 = 001;
    println!("ID: {}", ID);

    let (my_name, my_age) = ("Shubham",21);
    println!("{}, {}", my_name, my_age);
}