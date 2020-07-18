fn is_vowel_or_consoant(a: char) -> char {
    match a {
        'a' | 'e' | 'i' | 'o' | 'u' => 'v',
        'A' | 'E' | 'I' | 'O' | 'U' => 'v',
        _ => 'c',
    }
}

fn main() {
    let name: &'static str = "Ilgsson";
    let mut count_vowel = 0;
    let mut count_consoant = 0;

    for i in name.chars() {
        match is_vowel_or_consoant(i) {
            r @ 'v' => println!("{}", r), // vinculação de match
            r @ 'c' => println!("{}", r),
            _ => (),
        }
    }
}
