#![allow(unused)]

use core::num;
use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

mod ifelse;
mod data_types;
mod math;
mod random_number;
mod ternary_op;
mod mat;
mod arrays;
mod tuple;
mod strings;
mod casting;
mod enu;
mod vector;
mod functions;
mod generics;
mod apicall;

fn main() {

    // Print operations and input from terminal.

    println!("What is your name?");
    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin().read_line(&mut name)
        .expect( "Didn't Receive Input");
    println!("Hello {}! {}", name.trim_end(), greeting);

    const ONE_MIL: u32 = 1_00_000;
    const PI: f32 = 3.14592;
    let age= "47";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
    println!();

    // Data Types

    // Rust if statically typed programming language. Most of the time the type will be autogenerated in VScode. 
    // Refer below to manually insert types. 

    data_types::data();

    // Math in Rust

    math::math();

    // Generate a random number with a Rust crate. (Packages are called as crate in Rust !)

    random_number::random();

    // Conditional Statements. 

    // If else 
    // Below is a function call from another file. 
    // ifelse - name of the file followed by "funct()" name of the function in other file. Go to other file for function definition.

    ifelse::funct();

    // Terniary operator

    ternary_op::tern();

    // Match statement

    mat::mat();

    // Arrays

    arrays::array();

    // Tuple

    tuple::tuple();

    // Strings

    strings::string();

    // Casting : Converting from one type to another

    casting::casting();

    // Enumurated data types : enum

    enu::enu();

    // Vectors

    // Vectors are like arrays. They can grow if they are mutable. 
    // ! important - Vectors can only store values of the same type. 

    vector::vect();

    // Functions in rust

    functions::functions();

    // Generics

    generics::gen();

    // API call
    apicall::api();

}
 