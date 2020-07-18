// mod chars;
// mod booleans;
// mod numeric_values;
// mod arrays_slices;
// mod tuples;
// mod enums;

mod test {

    // private module
    fn message() {
        println!("Show message");
    }

    // public module
    pub fn sum(a: i32, b: i32) -> i32 {
        a + b
    }
}

// nomeia um módulo
use test::sum as sum2;

fn main() {
    // chars::run();
    // booleans::run();
    // numeric_values::run();
    // arrays_slices::run();
    // tuples::run();
    // enums::run();

    println!("soma : {}", sum2(2, 3));

    // Não conseguimos acessar o módulo message
    // pq ele não é público
}
