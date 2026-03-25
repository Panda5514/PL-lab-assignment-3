use std::env;
use std::fs::File;
use std::io::prelude::*;
use sexp::Atom::*;
use sexp::*;
use std::collections::HashMap;

// --- Task 1: Complete AST with Extensions ---
#[derive(Debug, Clone)]
pub enum Expr {
    Num(i64, usize),        // Changed to i64 to handle large constants for overflow testing
    Bool(bool, usize),      
    Var(String, usize),     
    Input(i32, usize),      // Extension: Multiple Inputs 
    Let(Vec<(String, Expr)>, Box<Expr>, usize),
    UnOp(UnOp, Box<Expr>, usize),
    BinOp(BinOp, Box<Expr>, Box<Expr>, usize),
    If(Box<Expr>, Box<Expr>, Box<Expr>, usize),
    Block(Vec<Expr>, usize),
    Loop(Box<Expr>, usize),
    Break(Box<Expr>, usize),
    Set(String, Box<Expr>, usize),
    Print(Box<Expr>, usize), // Extension: Print Statement 
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp { Add1, Sub1, Negate, IsNum, IsBool }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp { Plus, Minus, Times, Less, Greater, LessEq, GreaterEq, Equal }

fn new_label(label_counter: &mut i32, name: &str) -> String {
    *label_counter += 1;
    format!("{}_{}", name, label_counter)
}

// --- Task 1 & Extension: Parser with Line Numbers ---
fn parse_expr(s: &Sexp, line: usize) -> Expr {
    match s {
        Sexp::Atom(I(n)) => Expr::Num(*n, line), // Sexp::Atom(I) is already i64
        Sexp::Atom(S(id)) if id == "true" => Expr::Bool(true, line),
        Sexp::Atom(S(id)) if id == "false" => Expr::Bool(false, line),
        Sexp::Atom(S(id)) if id == "input" => Expr::Input(0, line),
        Sexp::Atom(S(id)) => Expr::Var(id.clone(), line),
        Sexp::List(vec) => {
            let op = match &vec[0] {
                Sexp::Atom(S(s)) => s.as_str(),
                _ => panic!("Line {}: Expected operator", line),
            };
            match op {
                "let" => {
                    let Sexp::List(bindings) = &vec[1] else { panic!("Line {}: Invalid let", line) };
                    let mut binds = Vec::new();
                    for b in bindings {
                        let Sexp::List(bind) = b else { panic!("Line {}: Invalid binding", line) };
                        let Sexp::Atom(S(name)) = &bind[0] else { panic!("Line {}: Expected name", line) };
                        binds.push((name.clone(), parse_expr(&bind[1], line)));
                    }
                    Expr::Let(binds, Box::new(parse_expr(&vec[2], line)), line)
                }
                "input" => {
                    let idx = if vec.len() > 1 {
                        let Sexp::Atom(I(i)) = vec[1] else { panic!("Index must be int") };
                        i as i32
                    } else { 0 };
                    Expr::Input(idx, line)
                }
                "if" => Expr::If(Box::new(parse_expr(&vec[1], line)), Box::new(parse_expr(&vec[2], line)), Box::new(parse_expr(&vec[3], line)), line),
                "loop" => Expr::Loop(Box::new(parse_expr(&vec[1], line)), line),
                "break" => Expr::Break(Box::new(parse_expr(&vec[1], line)), line),
                "set!" => {
                    let Sexp::Atom(S(name)) = &vec[1] else { panic!("Expected name") };
                    Expr::Set(name.clone(), Box::new(parse_expr(&vec[2], line)), line)
                }
                "block" => Expr::Block(vec[1..].iter().map(|e| parse_expr(e, line)).collect(), line),
                "print" => Expr::Print(Box::new(parse_expr(&vec[1], line)), line),
                "add1" => Expr::UnOp(UnOp::Add1, Box::new(parse_expr(&vec[1], line)), line),
                "sub1" => Expr::UnOp(UnOp::Sub1, Box::new(parse_expr(&vec[1], line)), line),
                "negate" => Expr::UnOp(UnOp::Negate, Box::new(parse_expr(&vec[1], line)), line),
                "isnum" => Expr::UnOp(UnOp::IsNum, Box::new(parse_expr(&vec[1], line)), line),
                "isbool" => Expr::UnOp(UnOp::IsBool, Box::new(parse_expr(&vec[1], line)), line),
                "+" => Expr::BinOp(BinOp::Plus, Box::new(parse_expr(&vec[1], line)), Box::new(parse_expr(&vec[2], line)), line),
                "-" => Expr::BinOp(BinOp::Minus, Box::new(parse_expr(&vec[1], line)), Box::new(parse_expr(&vec[2], line)), line),
                "*" => Expr::BinOp(BinOp::Times, Box::new(parse_expr(&vec[1], line)), Box::new(parse_expr(&vec[2], line)), line),
                "<" => Expr::BinOp(BinOp::Less, Box::new(parse_expr(&vec[1], line)), Box::new(parse_expr(&vec[2], line)), line),
                ">" => Expr::BinOp(BinOp::Greater, Box::new(parse_expr(&vec[1], line)), Box::new(parse_expr(&vec[2], line)), line),
                "<=" => Expr::BinOp(BinOp::LessEq, Box::new(parse_expr(&vec[1], line)), Box::new(parse_expr(&vec[2], line)), line),
                ">=" => Expr::BinOp(BinOp::GreaterEq, Box::new(parse_expr(&vec[1], line)), Box::new(parse_expr(&vec[2], line)), line),
                "=" => Expr::BinOp(BinOp::Equal, Box::new(parse_expr(&vec[1], line)), Box::new(parse_expr(&vec[2], line)), line),
                _ => panic!("Line {}: Unknown operator {}", line, op),
            }
        }
        _ => panic!("Line {}: Invalid expression", line),
    }
}

// --- Tasks 2-7 & Extensions: Code Generation ---
fn compile_expr(expr: &Expr, env: &HashMap<String, i32>, stack_offset: i32, label_counter: &mut i32, break_target: &Option<String>) -> String {
    match expr {
        // Tagged Representation: Numbers shifted left by 1 [cite: 1, 11]
        Expr::Num(n, _) => format!("mov rax, {}", *n << 1),
        // Booleans: true = 3, false = 1 [cite: 1, 12, 13]
        Expr::Bool(b, _) => format!("mov rax, {}", if *b { 3 } else { 1 }),
        Expr::Input(idx, _) => format!("mov rax, [rdi + {}]", idx * 8), 
        Expr::Var(name, line) => {
            let offset = env.get(name).expect(&format!("Line {}: Unbound variable {}", line, name));
            format!("mov rax, [rsp {}]", offset)
        }
        
        Expr::Print(e, _) => {
            let e_code = compile_expr(e, env, stack_offset, label_counter, break_target);
            format!("{}\n  mov rdi, rax\n  call snek_print", e_code)
        }

        Expr::UnOp(op, e, _) => {
            let e_code = compile_expr(e, env, stack_offset, label_counter, break_target);
            match op {
                UnOp::Add1 => format!("{}\n  test rax, 1\n  jnz error_invalid_argument\n  add rax, 2\n  jo error_overflow", e_code),
                UnOp::Sub1 => format!("{}\n  test rax, 1\n  jnz error_invalid_argument\n  sub rax, 2\n  jo error_overflow", e_code),
                UnOp::Negate => format!("{}\n  test rax, 1\n  jnz error_invalid_argument\n  neg rax", e_code),
                UnOp::IsNum => format!("{}\n  and rax, 1\n  xor rax, 1\n  shl rax, 1\n  or rax, 1", e_code),
                UnOp::IsBool => format!("{}\n  and rax, 1\n  shl rax, 1\n  or rax, 1", e_code),
            }
        }

        Expr::BinOp(op, l, r, _) => {
            let l_code = compile_expr(l, env, stack_offset, label_counter, break_target);
            let r_code = compile_expr(r, env, stack_offset - 8, label_counter, break_target);
            // Task 4: Runtime Type Checking [cite: 2, 25, 26]
            let check_nums = "  mov rcx, rax\n  or rcx, rbx\n  test rcx, 1\n  jnz error_invalid_argument";
            
            let op_asm = match op {
                BinOp::Plus => "add rax, rbx\n  jo error_overflow".to_string(), // Extension: Overflow 
                BinOp::Minus => "sub rbx, rax\n  mov rax, rbx\n  jo error_overflow".to_string(),
                BinOp::Times => "sar rax, 1\n  imul rax, rbx\n  jo error_overflow".to_string(),
                BinOp::Equal => "xor rax, rbx\n  test rax, 1\n  jnz error_invalid_argument\n  test rax, rax\n  setz al\n  movzx rax, al\n  shl rax, 1\n  or rax, 1".to_string(),
                _ => { 
                    let cc = match op { BinOp::Less => "l", BinOp::Greater => "g", BinOp::LessEq => "le", BinOp::GreaterEq => "ge", _ => unreachable!() };
                    format!("cmp rbx, rax\n  set{} al\n  movzx rax, al\n  shl rax, 1\n  or rax, 1", cc)
                }
            };
            format!("{}\n  mov [rsp {}], rax\n{}\n  mov rbx, [rsp {}]\n{}\n  {}", l_code, stack_offset, r_code, stack_offset, if *op != BinOp::Equal { check_nums } else { "" }, op_asm)
        }

        Expr::If(c, t, e, _) => { // Task 3: If [cite: 8, 11, 19, 21]
            let (l_else, l_end) = (new_label(label_counter, "else"), new_label(label_counter, "end"));
            format!("{}\n  cmp rax, 1\n  je {}\n{}\n  jmp {}\n{}:\n{}\n{}:", 
                compile_expr(c, env, stack_offset, label_counter, break_target), l_else,
                compile_expr(t, env, stack_offset, label_counter, break_target), l_end, l_else,
                compile_expr(e, env, stack_offset, label_counter, break_target), l_end)
        }

        Expr::Loop(b, _) => { // Task 5: Loop [cite: 8, 27, 28]
            let (l_start, l_end) = (new_label(label_counter, "loop"), new_label(label_counter, "break"));
            format!("{}:\n  {}\n  jmp {}\n{}:", l_start, compile_expr(b, env, stack_offset, label_counter, &Some(l_end.clone())), l_start, l_end)
        }

        Expr::Break(e, _) => format!("{}\n  jmp {}", compile_expr(e, env, stack_offset, label_counter, break_target), break_target.as_ref().expect("Break outside loop")),
        
        Expr::Set(name, val, _) => { // Task 1 & Extension: Mutation [cite: 9, 31]
            let off = env.get(name).expect("Unbound var");
            format!("{}\n  mov [rsp {}], rax", compile_expr(val, env, stack_offset, label_counter, break_target), off)
        }

        Expr::Block(es, _) => es.iter().map(|e| compile_expr(e, env, stack_offset, label_counter, break_target)).collect::<Vec<_>>().join("\n  "),
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut input = String::new();
    File::open(&args[1])?.read_to_string(&mut input)?;
    let expr = parse_expr(&parse(&input).expect("Parse error"), 1);
    let mut counter = 0;
    let code = compile_expr(&expr, &HashMap::new(), -8, &mut counter, &None);
    
    let asm = format!("
section .text
extern snek_error, snek_print
global our_code_starts_here
our_code_starts_here:
  push rbp
  mov rbp, rsp
  {}
  mov rsp, rbp
  pop rbp
  ret
error_invalid_argument: mov rdi, 1\n  call snek_error
error_overflow: mov rdi, 2\n  call snek_error", code);
    File::create(&args[2])?.write_all(asm.as_bytes())
}