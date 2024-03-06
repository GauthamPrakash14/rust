// Vectores are more like arrays when they are mutable. It can store data of same type. 

pub fn vect(){
    println!();
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1,2,3,4,5];
    vec2.push(6); // Push a value to the end of the vector.
    println!("1st : {}", vec2[0]);
    let second = &vec2[1];
    println!("Second : {}", second);
    match vec2.get(1){
        Some(second) => println!("2nd : {}", second),
        None => println!("No second value")
    }

    // Note in the above match, get() is used to get value at a specific index.

    for i in &mut vec2{
        *i *= 2;
    }

    for i in &vec2{
        println!("{}", i);
    }

    println!("Length of vector : {}", vec2.len());
    println!("Pop : {:?}", vec2.pop()); // Poping a value at the end of the vector

    // Note the use of {:?} in the above print statement 
    // :? is a placeholder inside the {} which helps to debug the output.
    // it Basically tells what type the output is.

}