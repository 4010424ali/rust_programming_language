// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    // hello is not change. This string has the fixed length
  let hello = "I am going to School in the morning ";
  println!("{}", hello );

   // Change the length the String 
  let mut hello2 = String::from("Can Change length of the string ");
  // Push Character
  hello2.push('W');
  
  // Push String 
  hello2.push_str("and making money");

  // Capacity of bytes
  println!("The Capacity {}", hello2.capacity());

  // Empty Empty 
  println!("Is empty {}", hello.is_empty());

  // Contains World 
  println!("contains 'going '{}",hello.contains("going"));

  // Replace World in string 
  println!("Replace: {}", hello2.replace("going", "Went"));

  // Loop through word white-space 
  for word in hello.split_whitespace() {
      println!("{}", word);
  }

  // Create String with capacity 
  let mut string_with_capacity = String::with_capacity(10);
  string_with_capacity.push('a');
  string_with_capacity.push_str("Going to");

  println!("{}", string_with_capacity);

 // Asesstion testing 
 assert_eq!(9, string_with_capacity.len());
 assert_eq!(10, string_with_capacity.capacity());


}