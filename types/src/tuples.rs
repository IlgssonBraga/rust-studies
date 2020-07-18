pub fn run() {
    let data = ('a', "Hello", 7, true);

    let data2 = ("Ilgsson", 25);

    let data3 = ("Arlaine", 24);

    let info = [data2, data3];

    let (a0, a1, a2, a3) = data; // destruct

    println!("{}, {}, {}, {}", a0, a1, a2, a3);
    println!("{:?}", info);
    println!("{}", info[0].0);

    // Tuplas podem armazenar diferentes tipos de dados
}
