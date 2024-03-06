pub fn string(){

    println!();
    let mut str1 = String::new();
    str1.push('A');
    str1.push_str("Words");
    for word in str1.split_whitespace(){
        println!("{}", word);
    }
    println!();
    let str2 = str1.replace("A", "another");
    println!("{}", str2);

    // String of random different characters

    println!();

    // Convert string to vector

    let st3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort(); // Sort the string
    v1.dedup(); // Remove duplicate from string

    println!("Sort the string and remove duplicate from the string");
    for char in v1{
        println!("{}",char);
    }

    println!();

    // String to literal 

    let st4: &str = "random string";
    let mut st5 = st4.to_string();

    println!("String to literal : {}", st5);

    // String to byte array
    println!();

    let byte_arr1 = st5.as_bytes();
    //println!("Byte array : {}", byte_arr1);

    // Slicing string
    println!();

    let st6 = &st5[0..6];
    println!("String length: {}", st6.len());
    println!("After slicing: {}", st6);

    // Delete a mut string

    println!();
    st5.clear();
    println!("After clearing : {}",st5);

    // Combine string

    let st6 = String::from("Just some");
    let st7 = String::from("words");

    println!();
    println!("Combining strings by calling location reference");
    let st8 = st6 + &st7;
    println!("Combined string {}", st8);
    // println!("Original values of string:  {} , {}", &st6, st7);
    // let st9 = st6 + &st7;
    println!();

    for char in st8.bytes(){
        println!("{}", char);
    }

}