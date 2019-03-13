// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run(){
   
   let name = "Ali";
   let mut age = 37;
   println!("My name is {} and I am {}", name, age);

   age = 36;
   println!("My name is {} and I am {}", name, age);

  // Define Constant >>> In Constant Delaration letter all uppercase 
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // Assign Multiple variable 
  let (number, marks, total) = (12, 78, 1050);

  println!("The number {} and marks is {} and total marks {}", number, marks, total );


}