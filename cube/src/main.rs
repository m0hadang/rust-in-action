/*
 * 큐브 위성 : 전통적인 인공위성에 비해 소형인 인공위성.
 * 지상 관제소 : 관제사와 위성 간 중계 역할. 전파를 청취하여 군집에 속한 모든 위성의 상태 검사 및
 * 메시지를 주고 받는다. 여기서는 사용자와 위성 간 게이트웨이 역할.
 * 군집 : 궤도를 돌고 있는 위성 집합.
 * */

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}


fn main() {
    let sat_a = CubeSat { id: 0 };
    let sat_b = CubeSat { id: 1 };
    let sat_c = CubeSat { id: 2 };

    // moved ownership
    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

    //wait...

    // can't access.
//    let a_status = check_status(sat_a);
//    let b_status = check_status(sat_b);
//    let c_status = check_status(sat_c);
//    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
}
