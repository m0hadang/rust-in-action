#![allow(unused)]

fn this_is_binding() {
    let _a = 10; // this is binding
}

fn float_is_danger() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
    println!("[f32]");
    println!("0.1 + 0.2 = {:x}", (abc.0 + abc.1).to_bits());
    println!("0.3 = {:x}", (abc.2).to_bits());

    println!("[f64]");
    println!("0.1 + 0.2 = {:x}", (xyz.0 + xyz.1).to_bits());
    println!("0.3 = {:x}", (xyz.2).to_bits());

    let x: f32 = 1.0 / 0.0;
    println!("{}", x);
    assert!(x.is_infinite());

    let x = (-32.0_f32).sqrt();
    println!("{}", x);
    assert!(x.is_nan());
}

fn check_float_using_epsilon() {
    let result: f32 = 0.1 + 0.1;
    let desired: f32 = 0.2;
    let abs_diff = (desired - result).abs();
    assert!(abs_diff <= f32::EPSILON);
    assert!(result == desired);
}

//rust is expresion language
fn is_even_expression(n: i32) -> bool {
    n % 2 == 0
}

fn rust_is_expression_language() {
    let n = 1234;
    let desc = if is_even_expression(1234) {
        "even"
    } else {
        "odd"
    };
    println!("{} is {}", n, desc);

    let n = loop {
        break 123;
    };
    println!("break {}", n);
}

fn match_is_important_in_rust() {
    let haystack = [1, 2, 3, 4];
    for item in &haystack {
        let res = match item {
            2 | 3 => "hit",
            _ => "miss",
        };
        println!("result : {res}");
    }
}

fn ref_and_memory_address() {
    let a = 42;
    let r = &a; // r : mem address of a
    let _b = a + *r; // deref
}

fn add_with_lifetime<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}

fn life_annotation() {
    let a = 10;
    let b = 20;
    println!("life annotation : {}", add_with_lifetime(&a, &b));
}

fn add_with_generic<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

fn generic() {
    let f = add_with_generic(1.2, 3.4);
    let i = add_with_generic(1, 3);
    let du = add_with_generic(
        std::time::Duration::new(5, 0),
        std::time::Duration::new(10, 0),
    );
    println!("generid : {}, {}, {:?}", f, i, du);
}

struct Hostname(String); // new type
fn ex_new_type_patter(host: Hostname) {
    println!("connected to {}", host.0);
}
enum NewType {
    AType(i64),
    BType(String),
}
fn rust_has_two_type_enum_and_struct(new_type: NewType) {
    match new_type {
        NewType::AType(i) => {
            println!("{i}")
        }
        NewType::BType(s) => {
            println!("{s}")
        }
        _ => {}
    }
}

fn ex_print_from_utf8_lossy() {
    // some bytes, in a vector
    let sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = String::from_utf8_lossy(&sparkle_heart);
    assert_eq!("💖", sparkle_heart);
}

#[derive(Default, Debug, PartialEq)]
enum FileState {
    #[default]
    Open, // set Enum Default Value
    Closed,
}

fn overflow_panic() {
    // if do not optimize then panic !
    let mut i: u16 = 0;
    println!("{} ..", i);
    loop {
        i += 1000;
        print!("{}..", i);
        if i % 10000 == 0 {
            println!();
        }
    }
}

fn container_arguments_using_ref(buf: &[u8]) {
    println!("{buf:?}")
}

fn return_result() -> std::io::Result<()> {
    println!("return_result_and_unwrap");
    Ok(())
}

fn match_with_unwrap(key: &str) {
    match key {
        // can use with println, eprintln
        "aaa" => return_result().unwrap(),
        "ccc" => println!("ccc"),
        _ => eprintln!("not match key"),
    }
}

fn main() {
    this_is_binding();
    float_is_danger();
    check_float_using_epsilon();
    rust_is_expression_language();
    match_is_important_in_rust();
    ref_and_memory_address();
    life_annotation();
    generic();
    //ex_new_type_patter("aaa"); // protect wrong use(it's type check)
    ex_print_from_utf8_lossy();
    rust_has_two_type_enum_and_struct(NewType::AType(10));
    //overflow_panic();

    let vec_data = vec![0_u8, 1_u8, 2_u8];
    container_arguments_using_ref(&vec_data);
    let arr_data = [3_u8, 4_u8, 5_u8];
    container_arguments_using_ref(&arr_data);

    match_with_unwrap("aaa");
}
