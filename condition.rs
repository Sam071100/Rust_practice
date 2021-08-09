pub fn run() {
    let age: u8 = 19;
    let check_id: bool = false;
    let knows_person_of_age: bool =true;

    if age >= 21 && check_id || knows_person_of_age{
        println!("Bartender: What do you want to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave");
    } else {
        println!("Bartender: I'll need to see your ID");
    }

    //Shorthand if

    let is_of_age = if age >=21 {true} else {false};
    println!("Is of Age{}", is_of_age);
}