#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Q7(i8);
// 고정 소수점을 표현학 ㅣ위한 Q7
//
// tuple struct, 이름 없는 필드로 부터 생성되는 구조체
//
// Clone : .clone()으로 복제 가능. i8이 Clone 트레이트를 구현하기에 가능
// Copy :  소유권 오류가 나지 않도록 암묵적으로 값싸게 복사. 이동 의미를 이용하는 타입에서 복사
// 의미를 이용하는 타임으로 변경
// PartialEq : == 로 비교 가능
// Eq : 다른 타입과 비교 가능

impl From<f64> for Q7 {
    fn from(n: f64) -> Self {
        if n >= 1.0 {
            Q7(127)
        } else if n <= -1.0 {
            Q7(-128)
        } else {
            Q7((n * 128.0) as i8)
        }
    }
}

impl From<Q7> for f64 {
    fn from(n: Q7) -> f64 {
        (n.0 as f64) * 2_f64.powf(-7.0)
    }
}

impl From<f32> for Q7 {
    fn from(n: f32) -> Self {
        Q7::from(n as f64)
    }
}

impl From<Q7> for f32 {
    fn from(n: Q7) -> f32 {
        f64::from(n) as f32
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_of_bounds() {
        assert_eq!(Q7::from(10.), Q7::from(1.));
        assert_eq!(Q7::from(-10.), Q7::from(-1.));
    }
}
