pub fn tuple(){
    
    let my_tuple: (u8, String, f64) = (47, "Gauthan".to_string(), 50_000.00);
    println!();

    println!("Name : {}", my_tuple.1);
    let (v1,v2,v3)= my_tuple;
    println!("Age : {}", v1);
}