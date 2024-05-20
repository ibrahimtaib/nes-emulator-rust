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

    fn fetch_next_pc(&mut self) -> u8 {
        let next_instruction: u16 = self.pc;
        self.pc += 1;
        self.memory[next_instruction as usize]
    }

    fn get_addressing_mode(&mut self) {
    }

    fn decode_and_execute(&mut self, opcode: u8) {
        match opcode {
           0xA9 => {
               let param = self.fetch_next_pc();
               self.a = param;
               
               if self.a == 0 {
                   self.status.set(CpuStatus::ZERO);
               } else {
                   self.status.clear(CpuStatus::ZERO);
               }

               if self.a & 0x80 != 0 {
                   self.status.set(CpuStatus::NEGATIVE);
               } else {
                   self.status.clear(CpuStatus::NEGATIVE);
               }
           }
           _ => todo!(),
        } 
    }

    fn run(&mut self) {
        loop {
            let opcode = self.fetch_next_pc();
            self.decode_and_execute(opcode);
            // Implement interrupt handling and cycle management
        }
    }
}

fn main() {
    let mut cpu = CPU::new();
    cpu.run();
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
    fn test_fetch_opcode() {
        let mut cpu = CPU::new();
        cpu.memory[0x0000] = 0xA9; // LDA immediate opcode
        assert_eq!(cpu.fetch_next_pc(), 0xA9);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn test_lda_immediate() {
        let mut cpu = CPU::new();
        cpu.write(cpu.pc , 0x06);
        cpu.decode_and_execute(0xA9);
        assert_eq!(cpu.a, 0x06);
        assert!(!cpu.status.is_set(CpuStatus::NEGATIVE));
        assert!(!cpu.status.is_set(CpuStatus::ZERO));
    }

    #[test]
    fn test_lda_immediate_zero() {
        let mut cpu = CPU::new();
        cpu.write(cpu.pc , 0x00);
        cpu.decode_and_execute(0xA9);
        assert_eq!(cpu.a, 0x00);
        assert!(cpu.status.is_set(CpuStatus::ZERO));
    }
}
