use std::io;

fn main() {
    let mut num1: String = String::new().to_string();

    let mut num2: String = String::new().to_string();

    let fn_pointer: fn() = message;

    to_do_things(fn_pointer);

    println!("Entre com um valor: ");
    io::stdin().read_line(&mut num1);

    println!("Digite outro valor: ");
    io::stdin().read_line(&mut num2);

    let my_num: i32 = num1
        .trim()
        .parse()
        .expect("please give me correct string number!");

    let my_num2: i32 = num2
        .trim()
        .parse()
        .expect("please give me correct string number!");

    println!("{} + {} = {}", my_num, my_num2, sum(my_num, my_num2));
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn message() {
    println!("Hi, my name is Ilgsson");
}

fn to_do_things(function: fn()) {
    function()
}
