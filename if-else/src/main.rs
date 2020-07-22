fn main() {
    let some_u8_value = Some(2);
    // match some_u8_value {
    //     Some(3) => println!("three"),
    //     _ => println!("different three"),
    // }

    if let Some(3) = some_u8_value {
        println!("three")
    }

    println!("{}", Some(2) == 2)
}
