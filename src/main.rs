use std::io;
use std::fs::File;
use std::io::Write; // Import Write trait

fn script_creator(line: &str, file_name: &str) {
    let mut script = String::from("#!/bin/bash\n");
    script.push_str(line);
    file_creator(&script, file_name);
}

fn file_creator(content: &str, file_name: &str)) {
    let mut file = File::create(file_name + ".sh").expect("Unable to create file");
    file.write_all(content.as_bytes()).expect("Unable to write to file");
}

fn main() {
    println!("Digite uma linha para adicionar ao script: ");
    let mut line = String::new();
    let mut file_name = String::new();
    io::stdin().read_line(&mut line).expect("Falha ao ler linha");
    println!("Nome do arquivo: (não incluir o .sh)");
    io::stdin().read_line(&mut file_name).expect("Falha ao ler nome do arquivo");
    script_creator(&line, &file_name);
    println!("Script criado com sucesso!");
}
