pub fn casting(){
    println!();
    let int_u8 : u8 = 5;
    let int2_u8 : u8 = 4;
    let int3_32 : u32 = (int2_u8 as u32) + (int_u8 as u32);
    println!("Result after casting : {}", int3_32);
}