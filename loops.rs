pub fn run() {
    let mut count1 = 0;

    // infinite loop
    loop {
        count1 += 1;
        println!("Number: {}", count1);

        if count1 == 20 {
            break;
        }
    }

    let mut count = 0;

    // While Loop FizzBuzz
    while count <=100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        }
        else {
            println!("{}", count);
        }
        count += 1;
    }

    for x in 0..100 {
        println!("{}",x);
    }
}