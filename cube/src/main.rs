/*
 * 큐브 위성 : 전통적인 인공위성에 비해 소형인 인공위성.
 * 지상 관제소 : 관제사와 위성 간 중계 역할. 전파를 청취하여 군집에 속한 모든 위성의 상태 검사 및
 * 메시지를 주고 받는다. 여기서는 사용자와 위성 간 게이트웨이 역할.
 * 군집 : 궤도를 돌고 있는 위성 집합.
 * */

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

type Message = String;
#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

struct GroundStation;

impl CubeSat {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

impl GroundStation {
    fn send(
        &self,
        to: &mut CubeSat,
        msg: Message,
        ) {
        to.mailbox.messages.push(msg);
    }

}


#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat: CubeSat) -> CubeSat {
    println!("{:?}: {:?}", sat, StatusMessage::Ok);
    sat
}


fn main() {
    let sat_a = CubeSat { id: 0, mailbox: Mailbox { messages: vec![] }  };
    let sat_b = CubeSat { id: 1, mailbox: Mailbox { messages: vec!["hello there!".to_string()] }  };
    let sat_c = CubeSat { id: 2, mailbox: Mailbox { messages: vec![] }  };

    // moved ownership
    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);

    //wait...
    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);
}
