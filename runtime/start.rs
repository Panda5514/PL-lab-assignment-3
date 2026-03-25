use std::env;

#[link(name = "our_code")]
extern "C" {
    fn our_code_starts_here(input_ptr: *const i64) -> i64;
}

#[no_mangle]
extern "C" fn snek_error(errcode: i64) {
    match errcode {
        1 => eprintln!("runtime error: invalid argument"),
        2 => eprintln!("runtime error: overflow"),
        _ => eprintln!("runtime error: unknown error"),
    }
    std::process::exit(1);
}

#[no_mangle]
extern "C" fn snek_print(val: i64) -> i64 {
    if val % 2 == 0 { println!("{}", val >> 1); }
    else if val == 3 { println!("true"); }
    else if val == 1 { println!("false"); }
    else { println!("Unknown: {}", val); }
    val
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let inputs: Vec<i64> = args[1..].iter()
        .map(|s| s.parse::<i64>().expect("Input must be number") << 1)
        .collect();
    
    let result = unsafe { our_code_starts_here(inputs.as_ptr()) };
    snek_print(result);
}