use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let query = &args[2];

    println!("archivo: {}", filename);
    println!("buscar: {}", query);

    let content = fs::read_to_string(filename).expect("Ocurrio un error al leer el archivo");

    println!("{}", content);
}
