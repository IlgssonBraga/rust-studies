pub fn run() {
    // let data = ["Ilgsson", "25", "Rust"];

    // let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // let numbers2 = &numbers[..]; // pega a referencia de acordo com
    // // a quantidade solicitada

    // let numbers3 = &numbers[0..4];

    // // let data2 = [0; 5];

    // // println!("{:?}", data2);

    // // data[2] = 1;

    // println!(
    //     "My name is {}, i'm {} and i like {}.",
    //     data[0], data[1], data[2]
    // );

    // println!("len: {}", data.len());
    // println!("numbers 2: {:?}", numbers2);
    // println!("numbers 3: {:?}", numbers3);

    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for i in numbers.iter() {
        println!("{}", i);
    }
}
