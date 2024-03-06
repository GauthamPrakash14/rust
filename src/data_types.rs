pub fn data(){

    // 1. Integers

    // Unsigned integers : u8, u16, u32, u64, u128, usize
    // Signed integer : i8, i16, i32, i64, i128, isize
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max usize: {}", usize::MAX);
    println!();

    // 2. Float : f32, f64

    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);
    println!();

    // 3. Boolean

    let is_true = true;
    println!("Boolean value {}", is_true);
    println!();

    // Fun Fact ! If there is a variable that is unused and you want the compiler to not throw errors, Just give an "_" at the begenning of the variable. The compiler will then ignore the variable and will not give errors. 

    // 4. Char

    let my_grade = 'A';
    println!("Single Character {}", my_grade);
    println!();
    
}