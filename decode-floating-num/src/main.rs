const BIAS: i32 = 127; // 지수 편차
const RADIX: f32 = 2.0; // 밑수

fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();
    let exponent = bits >> 23; // 부호
    let sign = bits >> 31; // 지수부
    let fraction = bits & 0x7fffffff; // 가수부
    (sign, exponent, fraction)
}

fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    let signed_1 = (-1.0_f32).powf(sign as f32);

    let exponent = (exponent as i32) - BIAS; //지수 편차 계산
    let exponent = RADIX.powf(exponent as f32);

    let mut mantissa: f32 = 1.0;
    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = fraction & mask;
        if one_at_bit_i != 0 {
            let i = i as f32;
            let weight = 2_f32.powf(i - 23.0);
            mantissa += weight;
        }
    }
    (signed_1, exponent, mantissa)
}

fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}

fn main() {
    let src_n: f32 = 42.42;
    let (sign, exp, frac) = to_parts(src_n);
    let (sign_, exp_, mant) = decode(sign, exp, frac);
    let dst_n = from_parts(sign_, exp_, mant);

    println!("--------------------------------------------------------------");
    println!("{} -> {}", src_n, dst_n);
    println!("--------------------------------------------------------------");
    println!("field\t\t|as bits\t\t\t|as real number");
    println!("--------------------------------------------------------------");
    println!("sign\t\t|{:01b}\t\t\t\t|{}", sign, sign_);
    println!("exponent\t|{:01b}\t\t\t|{}", exp, exp_);
    println!("mantissa\t|{:01b}|{}", frac, mant);
}
