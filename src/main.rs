use std::io;
use std::string::String;

fn main() {
    println!("Choose conversion type:");
    println!("[1] Fahrenheit -> Celsius.");
    println!("[2] Celsius -> Fahrenheit.");

    let mut option = String::new();

    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line.");

    option.pop();

    if option == "1" {
        println!("Type tempereture( Fahrenheit ):");
    } else if option == "2" {
        println!("Type tempereture( Celsius ):");
    } else {
        println!("Sorry, this is wrong input.");
        return
    }

    let mut tempereture = String::new();

    io::stdin()
        .read_line(&mut tempereture)
        .expect("Falied to read line.");
    
    let tempereture: i32 = match tempereture.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Cannot parse input.");
            return
        },
    };

    if option == "1" {
        to_celsius(tempereture);
    } else if option == "2" {
        to_fahrenheit(tempereture);
    }
}

fn to_celsius(f: i32) {
    println!("{} C", (f - 32)*5/9);
}

fn to_fahrenheit(c: i32) {
    println!("{} F", c*9/5+32);
}