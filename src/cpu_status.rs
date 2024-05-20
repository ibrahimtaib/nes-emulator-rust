#[derive(Debug)]
#[derive(PartialEq)]
pub struct CpuStatus(u8);
impl CpuStatus {
    pub const CARRY : u8 = 0b1;
    pub const ZERO : u8 = 0b10;
    pub const INTERRUPTDISABLE : u8 = 0b100;
    pub const DECIMALMODE : u8 = 0b1000;
    pub const BREAK : u8 = 0b10000;
    pub const OVERFLOW : u8 = 0b1000000;
    pub const NEGATIVE : u8 = 0b10000000;

    pub fn new() -> Self {
        CpuStatus(0)
    }

    pub fn get(& self) -> u8 {
        self.0
    }
    pub fn set(&mut self, flags: u8) -> u8 {
        self.0 |= flags;
        self.0
    }

    pub fn clear(&mut self, flags: u8) -> u8 {
        self.0 &= !flags;
        self.0
    }
    
    pub fn is_set(&mut self, flags: u8) -> bool {
        assert!(flags!=0, "You must specify which bits to set");
        self.0 & flags == flags
    }
}

#[cfg(test)]
mod tests { 
    use super::*;
    #[test]
    fn test_clear_status() {
        let mut status: CpuStatus = CpuStatus::new();
        status.set(0xff);
        status.clear(CpuStatus::CARRY);
        assert_eq!(status.get(), 0xfe);
    }

    #[test]
    fn test_set_status() {
        let mut status: CpuStatus = CpuStatus::new();

        status.set(CpuStatus::CARRY | CpuStatus::NEGATIVE);
        assert_eq!(status.get(), 0x81);
    }
}
