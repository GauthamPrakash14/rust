use ::std::ops::Add;

// We can specify the data type to be used at a later time.
// Generics is used to store any type of data and can handle data based on the input.
// Any data type can be used as generics and operations can be performed.

fn get_sum<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

pub fn gen() {
    println!();
    println!("Generics");
    println!();
    println!("5 + 4 = {}", get_sum(5, 4));
    println!("5.2 + 7.4 = {}", get_sum(5.2, 7.4));
}
