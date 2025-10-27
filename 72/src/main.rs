use std::env;
use num_bigint::BigInt;
use num_traits::Num;

fn add_ix(a: &str, b: &str) -> String {
    let x = BigInt::from_str_radix(a.trim_start_matches("0x"), 16).unwrap();
    let y = BigInt::from_str_radix(b.trim_start_matches("0x"), 16).unwrap();
    format!("{:x}", x + y)
}

fn sub_ix(a: &str, b: &str) -> String {
    let x = BigInt::from_str_radix(a.trim_start_matches("0x"), 16).unwrap();
    let y = BigInt::from_str_radix(b.trim_start_matches("0x"), 16).unwrap();
    format!("{:x}", x - y)
}

fn mul_ix(a: &str, b: &str) -> String {
    let x = BigInt::from_str_radix(a.trim_start_matches("0x"), 16).unwrap();
    let y = BigInt::from_str_radix(b.trim_start_matches("0x"), 16).unwrap();
    format!("{:x}", x * y)
}

fn div_ix(a: &str, b: &str) -> String {
    let x = BigInt::from_str_radix(a.trim_start_matches("0x"), 16).unwrap();
    let y = BigInt::from_str_radix(b.trim_start_matches("0x"), 16).unwrap();
    format!("{:x}", x / y)
}

fn rem_ix(a: &str, b: &str) -> String {
    let x = BigInt::from_str_radix(a.trim_start_matches("0x"), 16).unwrap();
    let y = BigInt::from_str_radix(b.trim_start_matches("0x"), 16).unwrap();
    format!("{:x}", x % y)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: cargo run <hex1> <hex2> <OP>");
        return;
    }

    let a = &args[1];
    let b = &args[2];
    let op = &args[3].to_uppercase();

    let result = match op.as_str() {
        "ADD" => add_ix(a, b),
        "SUB" => sub_ix(a, b),
        "MUL" => mul_ix(a, b),
        "QUO" => div_ix(a, b),
        "REM" => rem_ix(a, b),
        _ => panic!("Unknown operator: {}", op),
    };

    println!("{}", result);
}
