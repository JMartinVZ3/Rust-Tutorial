// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Martin";
    let mut age = 18;

    println!("My Name is {} and I am {}", name, age);

    age = 19;
    
    println!("My Name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("{}", ID);

    // Assign multiple Variables
    let (my_name, my_age) = ("Martin", 18);
    println!("My Name is {} and I am {}", my_name, my_age);
}