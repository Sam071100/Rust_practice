#![allow(non_snake_case)]

mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod array;
mod vectors;
mod condition;
mod loops;
mod functions;
mod pointers;
mod structs;

fn main() {
    println!("Hello, world!");
    println!("New to Rust Programming language");

    let a =5;
    let b =20;
    println!("{}",a+b);
    println!("{}", a*b);


    for i in 1..101 {
        
        if i > 50 {
            println!("Shubham");
        }
        else {
            println!("{}", i);
        }
    }

    print::run();
    vars::run();
    types::run();
    strings::run();
    tuples::run();
    array::run();
    vectors::run();
    condition::run();
    loops::run();
    functions::run();
    pointers::run();
    structs::run();
}
