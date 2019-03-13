pub fn run(){
    let mut count = 1;

    // infinite loop
    // loop {
    //     count += 1;
    //     println!("{}", count);
    //     if count >= 10 {
    //         break;
    //     }
    // }

    // While loop

    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzeBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 2 == 0 {
            println!("Buzz");
        } else {
            println!("Number: {}", count);
        }

        count += 1;
    }
    // for range 
    for x in 0..99 {
         if x % 15 == 0 {
            println!("FizzeBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 2 == 0 {
            println!("Buzz");
        } else {
            println!("Number: {}", x);
        }
    }

} 