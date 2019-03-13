pub fn run(){
  greeting("Hello", "Ali Hamza");

  // In Rust we can bind the funtion value to variable
  let add_number = add(12, 12);
  println!("The sum is {}", add_number);

  // Closure 
  let get_nums = |n1: i32, n2: i32| n1 + n2;
  println!("Get sum {}", get_nums(5, 5));

}

fn greeting(greet: &str, name: &str){
    println!("{} {}, Nie to meet you ", greet, name);
}

fn add(n1 : i32, n2: i32) -> i32 {
    n1 + n2
}