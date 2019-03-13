use std::mem;
pub fn run(){
    let mut number:[i32; 5] = [1, 2, 3, 4, 5]; 
    // Re-assign Value
    number[3] = 43; 

    // Get the length of array 
    println!("The length of the array {}", number.len() );

    // Arrays are stack allocated
    println!("This array occupied {} bytes", mem::size_of_val(&number) );

    // Get slice
    let slice: &[i32] = &number[0..2]; 

    println!("{:?}", slice );

}