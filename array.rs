use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] =  [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    println!("{}", numbers[0]);

    numbers[2] = 20;

    println!("{}", numbers.len());

    for i in 0..5 {
        print!("{} ", numbers[i]);
    }
    println!("");
    //Arrays are stack allocated
    println!("Array occupies {} bytes",mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
} 