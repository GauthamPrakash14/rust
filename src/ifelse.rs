use core::num;
use std::io;

pub fn funct(){
    
    // Input from terminal.
    println!("Enter age : ");
    let mut age = String::new();
    io::stdin().read_line(&mut age)
        .expect("Failed to read");

    // The below code snippet is to parse input string to numeric. 
    // In the code below, the inputted age which was stored as string is parsed to store as num below. 
    // Note that error handling is done atmost everywhere possible.
    let agenum: u32 = match age.trim().parse(){
        Ok(num) =>num,
        Err(_) => {
           println!("Invalid input. Please enter a valid number.");
           return;
        }
    };

    // If else statements in rust.
    if(agenum >= 1) && (agenum <= 18){
        println!("Important Birthday");
    }else if (agenum == 21) || (agenum == 50) {
        println!("Important Birthday");
    }else if (agenum >= 65) {
        println!("Important Birthday");
    }else {
        println!("Not an important Birthday");
    }
    println!()
}