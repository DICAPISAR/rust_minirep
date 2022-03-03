use std::env;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    let config = Config::new(&args);

    println!("archivo: {}", config.filename);
    println!("buscar: {}", config.query);

    minigrep::run(config);
}
