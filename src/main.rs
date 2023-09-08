use std::process::exit;

mod parser;
mod ast;
mod codegen;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        println!("usage: wut <file>");
        exit(0);
    }

    let file = &args[0];
    let text = std::fs::read_to_string(file).unwrap();
    let ast = parser::parse(&text);

    if args.contains(&"--dump-ast".to_string()) {
        dbg!(&ast);
    }

    let wat = codegen::generate(&ast);
    println!("{wat}");
}