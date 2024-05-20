mod cpu_status;
use cpu_status::CpuStatus;

struct CPU {
    pc: u16,
    sp: u8,
    a: u8,
    x: u8,
    y: u8,
    status: CpuStatus,
    memory: [u8; 0x10000],
}

impl CPU {
    fn new() -> Self {
        CPU {
           pc: 0,
           sp: 0,
           a: 0,
           x: 0,
           y: 0,
           status: CpuStatus::new(),
           memory: [0; 0x10000], 
        }
    }

    fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn write(&mut self, addr: u16, value: u8) {
        (self).memory[addr as usize] = value; 
    }

    fn update_negative_and_zero_bits(&mut self, value: u8) {
        if value == 0 {
            self.status.set(CpuStatus::ZERO);
        } else {
            self.status.clear(CpuStatus::ZERO);
        }

        if value & 0x80 != 0 {
            self.status.set(CpuStatus::NEGATIVE);
        } else {
            self.status.clear(CpuStatus::NEGATIVE);
        }
    }

    fn lda(&mut self, value: u8) {
        self.a = value;
        self.update_negative_and_zero_bits(value)
    }

    fn interpret(&mut self, program: Vec<u8>) {
        loop {
            //let opcode = self.fetch_next_pc();
            let opcode = program[self.pc as usize];
            self.pc += 1;
            match opcode {
                0xA9 => {
                    let param = program[self.pc as usize];
                    self.pc += 1;
                    self.lda(param);
                }
                0x00 => break,
                _ => todo!(),
            } 
        }
    }
}

fn main() {
    let mut cpu = CPU::new();
    //cpu.run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_initialization() {
        let cpu = CPU::new();
        assert_eq!(cpu.pc, 0);
        assert_eq!(cpu.sp, 0);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.status.get(), 0);
        assert_eq!(cpu.memory[0], 0);
    }

    #[test]
    fn test_memory_read_write() {
        let mut cpu = CPU::new();
        cpu.write(0x1234, 0x56);
        assert_eq!(cpu.read(0x1234), 0x56);
    }

    #[test]
    fn test_lda_immediate() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xA9, 0x06,0x00]);
        assert_eq!(cpu.a, 0x06);
        assert!(!cpu.status.is_set(CpuStatus::NEGATIVE));
        assert!(!cpu.status.is_set(CpuStatus::ZERO));
    }

    #[test]
    fn test_lda_immediate_zero() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xA9, 0x00,0x00]);
        assert_eq!(cpu.a, 0x00);
        assert!(cpu.status.is_set(CpuStatus::ZERO));
    }
}
