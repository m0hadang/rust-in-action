#![allow(unused)]

struct Hostname(String); // new type

fn connect(host: Hostname) {
    println!("connected to {}", host.0);
}

fn main() {
    //connect("aaa"); // cant use
}
