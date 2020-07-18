enum Gender {
    Female,
    Male,
}

struct Person {
    name: &'static str,
    gender: Gender,
}

pub fn run() {
    let person1 = Person {
        name: "Ilgsson Braga",
        gender: Gender::Male,
    };

    let person2 = Person {
        name: "Arlaine Gabriela",
        gender: Gender::Female,
    };

    println!("{}", person1.name);
    println!("{}", person2.name);
}
