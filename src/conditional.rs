pub fn run() {
    let age: u8 = 28;
    let check_id: bool = true;

    //  if/Slse
    if age >= 21 && check_id {
        println!("What do you wants drink ");
    } else if age <= 18 && check_id {
        println!("You can not get drink");
    }

    // Shorthand if 

    let is_of_age = if age > 21 { true } else { false };
    println!("The age is {}", is_of_age);
}