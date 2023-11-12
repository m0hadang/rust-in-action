/*
 * 큐브 위성 : 전통적인 인공위성에 비해 소형인 인공위성.
 * 지상 관제소 : 관제사와 위성 간 중계 역할. 전파를 청취하여 군집에 속한 모든 위성의 상태 검사 및
 * 메시지를 주고 받는다. 여기서는 사용자와 위성 간 게이트웨이 역할.
 * 군집 : 궤도를 돌고 있는 위성 집합.
 * */

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

#[derive(Default)]
struct GroundStation {
    radio_freq: f64,
}

impl CubeSat {
    fn recv(&mut self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }
    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }
}

impl GroundStation {
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id }
    }
    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {
    let mut mail_box = Mailbox { messages: vec![] };
    let base = Rc::new(RefCell::new(GroundStation {
        ..Default::default()
    }));
    let sat_ids = fetch_sat_ids();
    for sat_id in sat_ids {
        let base = base.borrow();
        let msg = Message {
            to: sat_id,
            content: String::from("hello"),
        };
        base.send(&mut mail_box, msg);
    }
    let sat_ids = fetch_sat_ids();
    for sat_id in sat_ids {
        let base = base.borrow();
        let mut sat = base.connect(sat_id);
        let msg = sat.recv(&mut mail_box);
        println!("{:?}: {:?}", sat, msg);
    }

    {
        // scope is essential
        let mut base = base.borrow_mut();
        base.radio_freq = 10.1;
    }
    println!("freq : {}", base.borrow().radio_freq);
    {
        let mut base = base.borrow_mut();
        base.radio_freq = 5.2;
    }
    println!("freq : {}", base.borrow().radio_freq);
}
