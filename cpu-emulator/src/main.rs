// CHIP-8
// 니블 : 4 bit
//
// 0x73EE : 8 개의 니블로 구성

// [opcode group][cpu register][cpu register, 정수][byte 수, opcode 하위 그룹]
struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 0x1000], // 4K
    stack_frame: [u16; 16],
    stack_pointer: usize,
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;
        op_byte1 << 8 | op_byte2
    }
    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            let addr = opcode & 0x0FFF;

            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(addr),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }
    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack_frame = &mut self.stack_frame;
        if sp > stack_frame.len() {
            panic!("Stack overflow");
        }
        stack_frame[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }
    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        let addr = self.stack_frame[self.stack_pointer];
        self.position_in_memory = addr as usize;
    }
    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0, // nop code
        stack_frame: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5; // REGISTER 0
    cpu.registers[1] = 20; // REGISTER 1

    let mem = &mut cpu.memory;

    // opcode 0x2100, 0x100의 함수를 CALL
    mem[0x000] = 0x21;
    mem[0x001] = 0x00;
    // opcode 0x2100, 0x100의 함수를 CALL
    mem[0x002] = 0x21;
    mem[0x003] = 0x00;
    // HALT
    mem[0x004] = 0x00;
    mem[0x005] = 0x00;

    // 함수
    // opcode 0x8014, REGISTER 1 의 값을 레지스터 0 에 ADD
    mem[0x100] = 0x80;
    mem[0x101] = 0x14;
    // opcode 0x8014, REGISTER 1 의 값을 레지스터 0 에 ADD
    mem[0x102] = 0x80;
    mem[0x103] = 0x14;

    // RETRUN
    mem[0x104] = 0x00;
    mem[0x105] = 0xEE;

    cpu.run();

    //assert_eq!(cpu.registers[0], 45);

    println!("5 + 20 + 20 = {}", cpu.registers[0]);
}
