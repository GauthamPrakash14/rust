pub fn enu(){

    // In Rust, an enum, short for enumeration, is a custom data type that allows you to define a type by enumerating its possible values. 
    // Enums are particularly useful when you have a type that can only have a limited set of predefined values.

    println!();

    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    // In rust, inside an enum, you can also create a function. 

    // The impl keyword is primarily used to define implementations on types.
    // If we want to define a function inside an enum type, We should use impl.

    impl Day {
        fn is_weekend(&self) -> bool{
            match self{
                Day::Saturday | Day::Sunday => true,
                _=> false
            }
        }
    }

    let today:Day = Day::Monday;
    match today{
        Day::Monday => println!("Everybody hates monday"),
        Day::Tuesday => println!("Pancake day"),
        Day::Wednesday => println!("Donut day"),
        Day::Thursday => println!("Fried Chicken day"),
        Day::Friday => println!("Pizza day"),
        Day::Saturday => println!("Weekend day"),
        Day::Sunday => println!("Weekend day")
    }

    println!("Is it a  weekend {}", today.is_weekend());

}