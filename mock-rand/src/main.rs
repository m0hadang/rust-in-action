fn mock_rand_1(n: u8) -> f32 {
    // 단순 나누기를 이용한 난수값 생성
    // byte로 표현 가능한 가장 큰 수로 나누었다.
    (n as f32) / 255.0
}
fn mock_rand_2(n: u8) -> f32 {
    // 성능 느린 / 연산 대신 비트 연산으로 계산

    // 0b01111110(126) : -1의 지수부, 0.5 ~ 0.998
    let base: u32 = 0b0_01111110_00000000000000000000000;

    let large_n = (n as u32) << 15; // 가수부에서 상쉬 8 bit 만 사용하기 위해 이동
    let f32_bits = base | large_n; // 가수부 설정
    let m = f32::from_bits(f32_bits);
    2.0 * (m - 0.5)
}

fn main() {
    println!("rand : {}", mock_rand_1(0x0e));
    println!("rand : {}", mock_rand_2(0x0e));
}
