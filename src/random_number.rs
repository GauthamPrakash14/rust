use rand::Rng;

pub fn random(){
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random number : {}", random_num);
    println!();
}