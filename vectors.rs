use std::mem;
pub fn run() {
    
    let mut numbers: Vec<i32> = vec![1,2,3,4];

    println!("{:?}", numbers);

    println!("{}", numbers[0]);

    numbers[2] = 20;
    numbers.push(5);
    numbers.push(6);

    println!("{}", numbers.len());

    for i in 0..6 {
        print!("{} ", numbers[i]);
    }
    println!("");

    numbers.pop(); // Pops the last element of the vector

    for x in numbers.iter() {
        print!("{} ",x);
    }
    println!("");

    //Loop and mutate value
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("{:?}", numbers);
    //Arrays are stack allocated
    println!("Vector occupies {} bytes",mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}