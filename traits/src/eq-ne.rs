fn main() {
    let a = 7;
    let b = 8;

    println!(" a == b {}", a == b);
    println!(" a == b: {}", a == b);
    println!(" a != b: {}", a != b);
    println!(" a.ne(&b): {}", a.ne(&b));
}

// Traits é um typo do Rust que define uma forma bem específica para uma
// struct
// eq e ne são traits que tem por syntax sugar '==' e '!=', respectivamente
// Quando usamos eq ou ne devemos sempre pegar a referência da variável
// que queremos comparar
