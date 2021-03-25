use std::io::prelude::*;
fn main() {
    let mut buff: Vec<u8> = Vec::new();
    std::io::stdin().read_to_end(&mut buff).expect("Could not read stdin");
    let buff = String::from_utf8(buff).unwrap();

    let mut cap = false;
    let mut result = String::new();
    for c in buff.chars() {
        let to_push: String = if cap {c.to_uppercase().collect()}
                              else {c.to_lowercase().collect()};
        if c.is_alphabetic() {cap = !cap};
        result.push_str(&to_push);
    }
    std::io::stdout().write(result.as_bytes()).expect("Could not write stdout");
}
