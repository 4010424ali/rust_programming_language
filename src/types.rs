/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run(){
   // Default is "i32"
  let _x = 1; 

  // Defaoult is "f64"
  let _y = 2.45;

  // Add explicit type
  let _z: i64 = 65777838686836;

  // Find MAx size 
  println!("Max: {}", std::i32::MAX);
  println!("Max: {}", std::i64::MAX);

  // Boolean 
  let is_bol = true;

  // Get boolean from expression 

  let is_graeter = 10 < 5; 

  // Character Type
  let Charater = 'a';
  let face = '\u{1F600}';

  println!("{}", face);
  println!("{:?}",(_x, _y, _z, is_bol, is_graeter, Charater));

}