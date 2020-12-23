pub fn run() {
    //Print to console
    println!("Hello from the print.rs file!");

    // Basic Formatting
    println!("{} is from {}", "Martin", "Ecuador");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", 
    "Martin", "Ecuador", "code");

    // Named Arguments
    println!("{name} likes to play {activity}", name = "Martin",
     activity = "Football");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}