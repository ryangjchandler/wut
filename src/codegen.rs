use std::fmt::Display;

use crate::ast::{Program, Statement, Expression};

#[derive(Default)]
struct Wat {
    functions: Vec<String>,
    exports: Vec<(String, String)>,
}

impl Display for Wat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut module = "(module\n".to_string();

        for function in self.functions.iter() {
            module.push_str(function);
        }
        for (export, reference) in self.exports.iter() {
            module.push_str("(export \"");
            module.push_str(export);
            module.push_str("\" ");
            module.push_str(reference);
            module.push_str(")\n");
        }
        return write!(f, "{}", format!("{module})"));
    }
}

pub fn generate(program: &Program) -> String {
    let mut wat = Wat::default();
    for statement in program.iter() {
        generate_top_statement(statement, &mut wat);
    }
    return wat.to_string();
}

fn generate_top_statement(statement: &Statement, wat: &mut Wat) {
    match statement {
        Statement::Function { name, args, body, exported } => generate_function(name, args, body, *exported, wat),
        _ => unimplemented!()
    }
}

fn generate_function(name: &String, args: &Vec<String>, body: &Vec<Statement>, exported: bool, wat: &mut Wat) {
    let mut def = String::new();
    def.push_str("(func $");
    def.push_str(name);
    def.push(' ');

    for (i, arg) in args.iter().enumerate() {
        def.push_str("(param $");
        def.push_str(arg);
        def.push_str(" i64)");

        if i == args.len() - 1 {
            def.push(' ');
        }
    }

    def.push_str("(result i64)\n");
    for statement in body {
        generate_statement(statement, &mut def);
    }
    def.push_str(")\n");
    wat.functions.push(def);

    if exported {
        wat.exports.push((name.to_string(), format!("(func ${name})")));
    }
}

fn generate_statement(statement: &Statement, code: &mut String) {
    match statement {
        Statement::Return { value } => {
            if let Some(value) = value {
                generate_expression(value, code);
            }

            code.push_str("return\n");
        },
        Statement::If { condition, then, otherwise } => {
            generate_expression(condition, code);
            code.push_str("(if\n(then\n");
            for statement in then {
                generate_statement(statement, code);
            }
            code.push_str(")\n)\n");
        },
        _ => unimplemented!("{:?}", statement),
    }
}

fn generate_expression(expression: &Expression, code: &mut String) {
    match expression {
        Expression::Add { lhs, rhs } => {
            generate_expression(lhs, code);
            generate_expression(rhs, code);
            code.push_str("i64.add\n");
        },
        Expression::Subtract { lhs, rhs } => {
            generate_expression(lhs, code);
            generate_expression(rhs, code);
            code.push_str("i64.sub\n");
        },
        Expression::LessThan { lhs, rhs } => {
            generate_expression(lhs, code);
            generate_expression(rhs, code);
            code.push_str("i64.lt_u\n");
        },
        Expression::Integer { value } => {
            code.push_str("i64.const ");
            code.push_str(&value);
            code.push('\n');
        },
        Expression::Identifier { name } => {
            code.push_str("local.get $");
            code.push_str(name);
            code.push('\n');
        },
        Expression::Call { target, args } => {
            for arg in args {
                generate_expression(arg, code);
            }

            code.push_str("call $");
            code.push_str(target);
            code.push('\n');
        },
        _ => unimplemented!("{:?}", expression),
    }
}