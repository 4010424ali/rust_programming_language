pub fn run(){
    // Printing to console
    println!("Hello world from print.rs");

    // Formating 

    println!("Number: {}", 1);

    //  multiple placehold in formating
    println!("{} is from {}", "Ali","mandiala Chatha");

    // Positional Arguments
    println!("{0} is from {1} and {0} like {2}", "Ali", "Mandiala Chatha", "code");

    // Named Arguments 
    println!("{name} is play {activity}", name= "Ali", activity="Cricket");

    // Placeholder traits > this for print binary and Hexidecimal Value 
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placehold for debug traits > Printing the array, vector, tuple 
    println!("{:?}", (12, true, "Hello"));

    // Basic Math 
    println!("10 + 10 = {}", 10 + 10);
}