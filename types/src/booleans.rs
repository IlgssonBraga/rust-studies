pub fn run() {
    let value1 = true;
    let value2: bool = false;

    if value1 || value2 {
        println!("Some value");
    }

    if value1 && value2 {
        println!("Value 1 and value 2");
    }

    if !value1 {
        println!("Value 1 is false");
    } else {
        println!("Value 1 is true");
    }

    if !value2 {
        println!("Value 2 is false");
    } else {
        println!("Value 2 is true");
    }
}
