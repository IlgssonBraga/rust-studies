// #[allow(dead_code)]

// fn check_sit(nota: f32) -> () {
//     match nota {
//         0.0...2.99 => println!("Reprovado direto"),
//         3.0...6.99 => println!("Avaliação final"),
//         7.0...9.99 => println!("Aprovador por média"),
//         10.0 => println!("Aprovado com excelência"),
//         _ => println!("Invalid"),
//     }
// }

// // match parece com um switch

// fn main() {
//     let numero = 14;

//     match numero {
//         2...5 => println!("[2,5]"),
//         6...9 => println!("[6,9]"),
//         10 | 11 | 12 => println!("10, 11 ou 12"),
//         _ => println!("Nenhuma condição satisfeita"),
//     }

//     let nota: f32 = 10.0;

//     check_sit(nota);
// }

// enum Gender {
//     Male,
//     Female,
// }

// fn main() {
//     let gender = Gender::Male;

//     match gender {
//         Gender::Male => println!("Male"),
//         Gender::Female => println!("Female"),
//     }
// }

// fn check_tuple(a: (i32, i32)) -> () {
//     match a {
//         (0, 0) => println!("Ambos são 0"),
//         (0, _x) => println!("Primeiro é zero"),
//         (_x, 0) => println!("Segundo é zero"),
//         _ => println!("Não há zero"),
//     }
// }

// fn main() {
//     check_tuple((0, 2));
// }

// fn is_vowel_or_consoant(a: char) -> char {
//     match a {
//         'a' | 'e' | 'i' | 'o' | 'u' => 'v',
//         'A' | 'E' | 'I' | 'O' | 'U' => 'v',
//         _ => 'c',
//     }
// }

// fn main() {
//     let name: &'static str = "Ilgsson";
//     let mut count_vowel = 0;
//     let mut count_consoant = 0;

//     for i in name.chars() {
//         match is_vowel_or_consoant(i) {
//             'v' => count_vowel += 1,
//             'c' => count_consoant += 1,
//             _ => (),
//         }
//     }

//     println!(
//         "{} tem {} vogais e {} consoantes",
//         name, count_vowel, count_consoant
//     );
// }
