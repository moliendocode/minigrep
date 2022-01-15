use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Hay problemas con la consulta: {}", err);
        process::exit(1);
    });

    println!("Buscando {}", config.query);
    println!("En el archivo {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Hay problemas con la ejecución: {}", e);
        process::exit(1);
    }
}


