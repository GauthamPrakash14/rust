// Functions can be defined before or after the main.
// Here we are aiming to cover all the cases related to functions. 

// 1. Generic function

fn hello_function(){
    println!("Say hi to functions");
}

// 2. Functions with arguments. 

fn get_sum(x: i32, y: i32){  // arguments are passed. Also the type of the arguments are specified. 
    println!("{} + {} = {}", x, y, x+y);
}

// 3. Function with return value, 
// Note ! If we are returning a value, we need to specify which data type it is returning. 

fn get_product(x: i32, y: i32) -> i32 { // return data type are specified using -> oeprator followed by return data type. 
    let mut z = x*y;
    return z;
}

// 4. Shorthand return without using return keyword

fn get_product2(x: i32, y: i32) -> i32 {  
    x * y // Notice that there is no semicolun. This is a short hand return without using the return keyword
}

// 5. Multiple return values. 

fn get(x: i32) -> (i32, i32) {  
    return (x+1, x+2);
}

// 6. Passing a vector to a function

fn sum_list(list: &[i32]) -> i32{ // Notice the type of the argument changes. &[i32] refers to a slice. Since vector store elements in contigunious memory locations, a memory slice can be used to specify type. 
    let mut sum = 0;
    for &val in list.iter(){
        sum += &val;
    }

    sum
}


pub fn functions(){

    println!();
    println!("Functions");

    println!();

    hello_function(); // Function call

    println!();
    get_sum(3, 5);
    
    println!();
    println!("Get product : {}", get_product(13, 8));

    println!();
    println!("Get product2 : {}", get_product2(13, 8));

    println!();
    println!("Multiple return values : {:?}", get(5));

    // Initialise 2 variables together

    let (v1, v2) = get(7);
    let (p1, p2) = (2,5);
    println!();
    println!("Nums from function :  {},{}", v1, v2);
    println!("Nums multiple initialisation : {}, {}", p1, p2);

    // Passing a vector 

    let num_list = vec![4,6,3,6,7];
    println!();
    println!("Sum of list = {}", sum_list(&num_list));

}