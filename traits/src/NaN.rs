fn main() {
    let a = 0.0f64 / 0.0f64;
    let b = 0.0f64 / 0.0f64;
    let c = 0.0f32 / 0.0f32;
    let d = 0.0f32 / 0.0f32;

    println!("a = {}", a);
    println!("b = {}", b);
    println!("a == b:  {}", a == b);
    println!("c = {}", c);
    println!("d = {}", d);
    println!("c == d: {}", c == d);

    // Para o Rust cada NaN é diferente de outro
}
