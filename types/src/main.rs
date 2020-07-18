fn main() {
    let emoji: char = '\u{1F618}';
    let emoji2: char = '😘';
    let b: char = '9';
    let c: char = '0';
    let repr: String = emoji2.escape_unicode().collect();

    println!("{}", emoji);
    println!("is digit: {}", emoji.is_digit(10));
    println!("is digit: {}", emoji.is_digit(2));
    println!("b is 10-digit: {}", b.is_digit(10));
    println!("b is 2-digit: {}", b.is_digit(2));
    println!("c is 10-digit: {}", c.is_digit(10));
    println!("c is 2-digit: {}", c.is_digit(2));
    println!("escape unicode: {}", repr);
    println!("a is alphabetic: {}", 'a'.is_alphabetic());
    println!("' ' is alphabetic: {}", ' '.is_alphabetic());
    println!("'a' is lowercase: {}", 'a'.is_lowercase());
    println!("'a' is uppercase: {}", 'a'.is_uppercase());
    println!("'a' is alphanumeric: {}", 'a'.is_alphanumeric());
    println!("'1' is alphanumeric: {}", 'a'.is_alphanumeric());
    println!("'' is whitespace: {}", ' '.is_whitespace());
    println!("'1' is numeric: {}", '1'.is_numeric());
}
