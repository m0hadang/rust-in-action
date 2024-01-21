use std::borrow::Cow;
use std::ffi::CStr;
use std::mem::size_of;
use std::os::raw::c_char;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 103, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn test1() {
    let a: usize = 42;
    let b: &[u8; 10] = &B;
    let c: Box<[u8]> = Box::new(C);

    println!("a (an unsigned integer):");
    println!(" location: {:p}", &a);
    println!(" size: {:?} bytes", size_of::<usize>()); // 8
    println!(" value: {:?}", a);
    println!();

    println!("b (a refrence to B):");
    println!(" location: {:p}", &b);
    println!(" size: {:?} bytes", size_of::<&[u8; 10]>()); // 8
    println!(" value: {:p}", b);
    println!();

    println!("c (a box for c):");
    println!(" location: {:p}", &c);
    println!(" size: {:?} bytes", size_of::<Box<[u8]>>()); // 16
    println!(" value: {:p}", c);
    println!();

    println!("B (an array of 10 bytes):");
    println!(" location: {:p}", &B);
    println!(" size: {:?} bytes", size_of::<Box<[u8; 10]>>()); // 8
    println!(" value: {:?}", B);
    println!();

    println!("C (an array of 11 bytes):");
    println!(" location: {:p}", &C);
    println!(" size: {:?} bytes", size_of::<Box<[u8; 11]>>()); // 8
    println!(" value: {:?}", C);
    println!();
}
fn test2() {
    let a: i64 = 42;
    let a_ptr = &a as *const i64;
    let a_addr: usize = unsafe { std::mem::transmute(a_ptr) };
    println!("a: {} {:p}...0x{:x}", a, a_ptr, a_addr + 7)
}
fn test3() {
    let a = 42;
    let b: String;
    let c: Cow<str>;
    unsafe {
        let b_ptr = &B as *const u8 as *mut u8;
        b = String::from_raw_parts(b_ptr, 10, 10);
        let c_ptr = &C as *const u8 as *const c_char;
        c = CStr::from_ptr(c_ptr).to_string_lossy();
    }
    println!("a: {}, b: {}, c:{}", a, b, c);
}
fn main() {
    test1();
    test2();
    test3();
}
