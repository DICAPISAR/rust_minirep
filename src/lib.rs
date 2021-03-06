use std::fs;

pub struct Config {
    pub filename: String,
    pub query: String,
    pub info: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let filename = args[1].clone();
        let query = args[2].clone();
        let mut info = false;

        if args.contains(&"-info".to_string()) {
            info = true;
        }

        Config { filename, query, info }
    }
}

struct Result {
    number_line: String,
    line: String,
}

impl Result {
    fn new(number_line: String, line: String) -> Result {
        let n = number_line.clone();
        let l = line.clone();
        Result {number_line: n, line: l}
    }
}

pub fn run(config: Config) {
    let content = fs::read_to_string(config.filename).expect("Ocurrio un error al leer el archivo");
    let results = search(&config.query, &content);

    for result in results {
        if config.info {
            println!("Line: {}, Result: {}", result.number_line, result.line);
        } else {
            println!("{}", result.line);
        }

    }
}

fn search(query: &str, content: &str) -> Vec<Result> {
    let mut results = Vec::new();

    for (i, l) in content.lines().enumerate() {
        if l.contains(query) {
            let result = Result::new((i+1).to_string(), l.to_string());
            results.push(result);
        }
    }

    results
}