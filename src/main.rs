use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Buscando {}", query);
    println!("En el archivo {}", filename);

    let contents = fs::read_to_string(filename).expect("No se pudo leer el archivo");

    println!("El contenido del archivo es: \n{}", contents);
}
