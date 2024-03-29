pub fn array(){

    let arr_1 = [1,2,3,4,5,6,7,8,9];
    println!("1st : {}", arr_1[0]);
    println!("Length : {}", arr_1.len());

    // Loop through array

    let mut loop_idx = 0;
    loop{
        if arr_1[loop_idx] % 2 == 0{
            loop_idx +=1;
            continue;
        }
        if arr_1[loop_idx] == 9{
            break;
        }
        println!("Val : {}", arr_1[loop_idx]);
        loop_idx +=1;
    }

    // while loop

    loop_idx = 0;

    println!();

    println!("While loop");

    while loop_idx < arr_1.len(){
        println!("Val : {}", arr_1[loop_idx]);
        loop_idx +=1;
    }

    // For loop

    println!();
    println!("For loop");

    for val in arr_1.iter() {
        println!("Val : {}", val);
    }

    println!();

}