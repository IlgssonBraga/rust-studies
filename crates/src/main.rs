extern crate time;

fn main() {
    let d = time::now();

    let mut number = 7;

    const PI: f32 = 3.1428;

    println!("{}", number);

    println!("{}", PI);

    number = 17;

    const YEAR: i32 = 1900;

    let day: i32 = d.tm_mday;
    let month: i32 = d.tm_mon;
    let year: i32 = d.tm_year + YEAR;
    println!("{}", number);
    println!("Today is {}/{}/{}", day, month, year);
}
