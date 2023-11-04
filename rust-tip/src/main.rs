#![allow(unused)]

struct Hostname(String); // new type

fn ex_new_type_patter(host: Hostname) {
    println!("connected to {}", host.0);
}

fn ex_print_from_utf8_lossy() {
    // some bytes, in a vector
    let sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = String::from_utf8_lossy(&sparkle_heart);
    assert_eq!("💖", sparkle_heart);
}

fn main() {
    //ex_new_type_patter("aaa"); // protect wrong use(it's type check)
    ex_print_from_utf8_lossy();
}
