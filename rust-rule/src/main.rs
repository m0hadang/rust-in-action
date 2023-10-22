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

fn main() {
    this_is_binding();
    float_is_danger();
    check_float_using_epsilon();
    rust_is_expression_language();
    match_is_important_in_rust();
    ref_and_memory_address();
}
