use std::mem;
pub fn run(){
    let mut number: Vec<i32> = vec![1, 2, 3, 4, 5]; 
    // Re-assign Value
    number[3] = 43; 

    // Add to vector 

    number.push(5);
    number.push(6);
    number.push(7);

    // Get the length of a vetor 
    println!("The length of the vector {}", number.len() );

    // Vector are stack allocated
    println!("This vector occupied {} bytes", mem::size_of_val(&number) );

    // Get slice
    let slice: &[i32] = &number[0..2]; 
    println!("{:?}", slice );

    for item in number.iter() {
        println!("Number {}", item);
    }

    for item in number.iter_mut() {
        *item *= 2;
    }
    println!("The value {:?}", number);
}