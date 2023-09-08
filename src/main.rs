use std::process::exit;
use wasmer::{self, imports};

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

    if args.contains(&"--run".to_string()) {
        let mut store = wasmer::Store::default();
        let module = wasmer::Module::new(&store, &wat).unwrap();
        let imports = imports! {};
        let instance = wasmer::Instance::new(&mut store, &module, &imports).unwrap();
        let fib = instance.exports.get_function("fib").unwrap();
        let result = fib.call(&mut store, &[wasmer::Value::I64(30)]).unwrap();
        println!("{result:?}");
    } else {
        println!("{wat}");
    }
}