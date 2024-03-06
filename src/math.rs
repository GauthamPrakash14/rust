pub fn math(){
    // 1. Addition

    let num_1: f32 = 1.11111111111;
    println!("Addition");
    println!();
    println!("f32 : {}", num_1 + 0.11111111111);
    println!();
    // 32 bit will have 6 digits of precision. See output

    let num_2 = 1.11111111111;
    println!("f64 : {}", num_2 + 0.0012345678);
    println!();
    // 64 bit addition will have 16 digits of precision. See output

    // 2. Subtraction

    let num3 = 4.4444343;
    println!("Subtraction");
    println!();
    println!("f64 : {}", num3 - 3.3323454);
    println!();

    // All Math operation 

    let num_4 = 4;
    let num_5 = 5;
    println!("num1 + num2 : {}", num_4 + num_5);
    println!("num1 - num2 : {}", num_4 - num_5);
    println!("num1 * num2 : {}", num_4 * num_5);
    println!("num1 / num2 : {}", num_4 / num_5);
    println!("num1 % num2 : {}", num_4 % num_5);
    println!();
}