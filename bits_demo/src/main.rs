use std::io;

fn two_hex(n: char, m: char) -> u8 {
    let n = u8::from_str_radix(&n.to_string(), 16).unwrap();
    let m = u8::from_str_radix(&m.to_string(), 16).unwrap();
    return n * 16 + m;
}

fn custom_u8s_to_u32(vals: Vec<u8>) -> u32 {
    let mut ret: u32 = 0;
    for i in 0..vals.len() {
        ret += (vals[i] as u32) << (8 * i);
    }
    return ret;
}

fn main() {
    // Read a hex string like "0x1234"
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).unwrap();

    // Skip "0x"
    let mut cs = guess.chars();
    cs.next();
    cs.next();

    // Build vector of bytes
    let mut vals: Vec<u8> = Vec::new();
    let mut chars: Vec<char> = cs.filter(|c| *c != '\n').collect();

    // If odd length, pad with '0'
    if chars.len() % 2 != 0 {
        chars.insert(0, '0');
    }

    for pair in chars.chunks(2) {
        vals.push(two_hex(pair[0], pair[1]));
    }

    println!("Vector of bytes: {:?}", vals);

    // Combine into u32 (little-endian)
    let le_val = custom_u8s_to_u32(vals.clone());
    println!("Combined little-endian: {:#x}", le_val);

    // Combine into u32 (big-endian)
    let mut be_vals = vals.clone();
    be_vals.reverse();
    let be_val = custom_u8s_to_u32(be_vals);
    println!("Combined big-endian: {:#x}", be_val);
}

