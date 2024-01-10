use std::io;
// Import file
use std::fs::File;

fn script_creator (line: &str) {
    let mut script = String::new();
    script.push_str(line);
    // Turn script into a .sh file
    script.push_str("#!/bin/bash\n");
    // Create file
    file_creator(&script);
}

fn file_creator (content: &str) {
    let mut file = String::new();
    file.push_str(content);
    // Create file
    let mut file = File::create("script.sh").unwrap();
}

fn main() {
    let mut line = String::new();
    println!("Digite uma linha para adicionar ao script: ");
    std::io::stdin().read_line(&mut line).unwrap();
    script_creator(&line);
    println!("Script criado com sucesso!");
}
