use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let query = &args[2];

    println!("archivo: {}", filename);
    println!("buscar: {}", query);
}
