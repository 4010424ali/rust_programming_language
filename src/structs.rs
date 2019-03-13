// Structs - Used to create the custom data type

// Traditional struct
//  struct Color {
//      red: u8, 
//      blue: u8, 
//      green: u8
// }

//  Tuple Struct
// struct Color (u8, u8, u8);

struct Person { 
    first_name: String, 
    last_name: String
}

impl Person {
    //  Construct Person 

    fn new(first: &str, last: &str) -> Person{
        Person {
            first_name: first.to_string(), 
            last_name: last.to_string()
        }
    }
    // full name 
    fn first_name(&self) -> String {
        format!("{} {} ", self.first_name, self.last_name)
    }

    // Set the last name function
    fn set_last_name(&mut self, last: &str){
        self.last_name = last.to_string();
    }

    // Name to tuple 
    fn to_tuple(self) -> (String, String) {
       // No semi coln because we are vreturning the value 
       (self.first_name, self.last_name)
    }
}
pub fn run(){
//    let mut c = Color {
//        red: 255, 
//         blue: 0, 
//        green: 0
//    };

  // We can change the value of rthe struct
    // c.blue = 200;

//   println!("The Color {} {} {}", c.green, c.blue, c.red );

    // let mut c= Color(255, 0, 0);

    // c.0 = 23;

    // println!("The value of the color: {} {} {}", c.0, c.1, c.2)

    let mut p = Person::new("Ali", "Hamza");
    p.set_last_name("Chatha");
    println!("full_name {}", p.first_name());
    println!("Person tuple {:?}", p.to_tuple());
}