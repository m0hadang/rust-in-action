use std::error::Error;
use std::fs::File;
use std::net::Ipv6Addr;

// // compile error!
// // can't find stack size in compile time
// fn main() -> Result<(), Box<Error>> { // return diffrent error
//     let _a = "::1".parse::<Ipv6Addr>()?;// return Ipv6Addr Error
//     let _b = File::open("invisible.txt")?;// return File Error
//
//     Ok(())
// }

// fix compile error
fn main() -> Result<(), Box<dyn Error>> {
    // use dynamic type
    // disadvantage
    // - runtime cost
    // - erased original type
    let _a = ":0".parse::<Ipv6Addr>()?;
    let _b = File::open("invisible.txt")?;

    Ok(())
}
