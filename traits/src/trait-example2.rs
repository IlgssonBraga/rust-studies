trait Cars {
    fn new(marca: &'static str, modelo: &'static str) -> Self;

    fn marca(&self) -> &'static str;

    fn modelo(&self) -> &'static str;

    fn message(&self) -> &'static str;
}

#[derive(Debug)]
struct Car {
    marca: &'static str,
    modelo: &'static str,
}

impl Cars for Car {
    fn new(marca: &'static str, modelo: &'static str) -> Car {
        Car { marca, modelo }
    }

    fn marca(&self) -> &'static str {
        self.marca
    }

    fn modelo(&self) -> &'static str {
        self.modelo
    }

    fn message(&self) -> &'static str {
        "Trait Cars implementada"
    }
}

fn main() {
    let ferrari1: Car = Cars::new("Ferrari", "F50");

    println!("{}", ferrari1.message());
    println!("Marca: {}", ferrari1.marca());
    println!("Modelo: {}", ferrari1.modelo());
    println!("{:?}", ferrari1);
}
