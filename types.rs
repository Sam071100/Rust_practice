pub fn run() {

    /*
    Primitive Types-->
    Integers: u8, u16, u32, i32, u64, i64, u128, i128 (Number of bits they take in memory)
    Floats: f32, f64
    Boolean (bool)
    Characters (char)
    Tuples
    Arrays
    */

    // BY default x is i32
    let x = 1;
    println!("{}",x);
    
    // By default y is f64
    let y = 2.5;
    println!("{}",y);

    let z: i64 = 45054451545;
    println!("{}",z);

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    let is_active: bool = true;
    let is_greater: bool = 1 > 5;

    let a1 = 's';
    let face = '\u{1F601}'; // Emoji
    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}