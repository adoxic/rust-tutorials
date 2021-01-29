use std::io;

fn main() {
    println!("Is your temperature Fahrenheit or Celsius?  Enter F or C to select");

    let mut unit = String::new();

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line");
    
    println!("Input the number of degrees");

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");
        
    let temp_int: i32 = temp.trim().parse().unwrap();

    if unit.to_lowercase().starts_with("f") {
        println!("{}F is {}C", temp_int, (temp_int - 32) * 5/9);
    } else {
        println!("{}C is {}F", temp_int, (temp_int * 9/5) + 32);
    }
    //git test
}
