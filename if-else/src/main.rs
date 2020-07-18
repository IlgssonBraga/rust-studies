fn check_sit(nota: f32) {
    if nota < 3.0 {
        println!("Reprovado direto");
    } else if nota >= 3.0 && nota < 7.0 {
        println!("Avaliação final");
    } else if nota >= 7.0 && nota < 10.0 {
        println!("Aprovado por média");
    } else if nota == 10.0 {
        println!("Aprovado com excelência");
    }
}

fn main() {
    let nota: f32 = 10.0;

    check_sit(nota);

    // let x = if 10 + 5 == 15 {
    //     "10 + 5 = 15"
    // } else {
    //     "10 + 5 != 15"
    // };

    // println!("{}", x);
}
