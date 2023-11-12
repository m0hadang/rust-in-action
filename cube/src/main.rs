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

fn main() {
    let base = GroundStation {};
    let mut sat_a = CubeSat { id: 0, mailbox: Mailbox { messages: vec![] }  };

    println!("1: {:?}", sat_a);
    base.send(&mut sat_a, Message::from("hello there!!"));
    println!("2: {:?}", sat_a);
    let msg = sat_a.recv();
    println!("3: {:?}", sat_a);
    println!("msg : {:?}", msg);
}
