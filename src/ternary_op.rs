pub fn tern(){
    let mut my_age = 47;
    let can_vote = if my_age >= 18 {
        true
    }else{
        false
    };
    println!("Can vote : {}", can_vote);
}