pub fn run() {
    // // de -2⁽n - 1) até 2⁽n - 1⁾ - 1
    // println!("i8 vai de {} até {}", i8::min_value(), i8::max_value());
    // println!("i16 vai de {} até {}", i16::min_value(), i16::max_value());
    // println!("i32 vai de {} até {}", i32::min_value(), i32::max_value());
    // println!("i64 vai de {} até {}", i64::min_value(), i64::max_value());
    // println!(
    //     "i128 vai de {} até {}",
    //     i128::min_value(),
    //     i128::max_value()
    // );
    // // de 0 até 2⁽n⁾ - 1
    // println!("u8 vai de {} até {}", u8::min_value(), u8::max_value());
    // println!("u16 vai de {} até {}", u16::min_value(), u16::max_value());
    // println!("u32 vai de {} até {}", u32::min_value(), u32::max_value());
    // println!("u64 vai de {} até {}", u64::min_value(), u64::max_value());
    // println!(
    //     "u128 vai de {} até {}",
    //     u128::min_value(),
    //     u128::max_value()
    // );

    let number: i8 = 1;

    println!("count ones: {}", number.count_ones());
    println!("count zeros: {}", number.count_zeros());
    println!("rotate left: {}", number.rotate_left(7));
    println!("rotate right: {}", number.rotate_right(8));
    println!("swap bytes: {}", number.swap_bytes());
}
