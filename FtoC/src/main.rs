use std::io;

fn main() {
    println!("Select what Fibonacci number you want starting with 0 and 1");

    let mut nth = String::new();

    let mut i = 1;
    let mut fib_1 = 0;
    let mut fib_2 = 1;
    
    io::stdin()
        .read_line(&mut nth)
        .expect("Failed to read line");

    loop {
    
        let nth: u32 =  match nth.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if nth == 1 {
            println!("The 1st number in the sequence is 0");
            break;
        }
        if nth == 2 {
            println!("The 2nd number in the sequence is 1");
            break;
        }
        if nth == 3 {
            println!("The 3rd number in the sequence is 1");
            break;
        }
        if i == nth {
            println!("The {}th number in the sequence is {}", nth, fib_1 + fib_2);
            break;
        }

        i = i + 1;
        let temp = fib_1 + fib_2;
        fib_1 = fib_2;
        fib_2 = temp;

    }
    //git test
}
