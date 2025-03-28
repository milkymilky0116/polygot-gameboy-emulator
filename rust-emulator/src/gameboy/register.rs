// - 8비트 레지스터
// - A: 누산기 (Accumulator) - 계산 결과를 저장
//   - B, C, D, E, H, L - 범용 레지스터
//   - F: 플래그 레지스터 - 연산 결과의 상태를 나타내는 플래그를 보관

pub struct Register {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: u8,

    pc: u16,
    sp: u16,
}

#[derive(Clone, Copy)]
pub enum Flag {
    Z = 0x80,
    N = 0x40,
    H = 0x20,
    C = 0x10,
}

impl Register {
    pub fn new() -> Self {
        let mut registers = Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            f: 0,
            pc: 0,
            sp: 0,
        };
        registers.reset();
        registers
    }
    // DMG 기준
    pub fn reset(&mut self) {
        self.a = 0x11;
        self.b = 0x00;
        self.c = 0x13;
        self.d = 0x00;
        self.e = 0xD8;
        self.h = 0x01;
        self.l = 0x4D;
        self.f = 0xB0;
        self.pc = 0x0100;
        self.sp = 0xFFFE;
    }

    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | ((self.f & 0xF0) as u16)
    }

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0x00F0) as u8;
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        // 하위 8비트만 추출
        self.l = (value & 0x00FF) as u8;
    }

    pub fn set_a(&mut self, value: u8) {
        self.a = value
    }

    pub fn set_b(&mut self, value: u8) {
        self.b = value
    }

    pub fn set_c(&mut self, value: u8) {
        self.c = value
    }

    pub fn set_d(&mut self, value: u8) {
        self.d = value
    }

    pub fn set_e(&mut self, value: u8) {
        self.e = value
    }

    pub fn set_h(&mut self, value: u8) {
        self.h = value
    }

    pub fn set_l(&mut self, value: u8) {
        self.l = value
    }

    pub fn set_f(&mut self, flag: u8) {
        self.f = flag & 0xF0
    }

    pub fn get_f(&self) -> u8 {
        self.f
    }

    pub fn inc_hl(&mut self) -> u16 {
        let temp = self.get_hl();
        self.set_hl(temp + 1);
        temp
    }

    pub fn dec_hl(&mut self) -> u16 {
        let temp = self.get_hl();
        self.set_hl(temp - 1);
        temp
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        let mask = flag as u8;
        self.f & mask > 0
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        let mask = flag as u8;
        match value {
            true => self.f |= mask,
            false => self.f &= !mask,
        }
        // F 레지스터의 하위 4비트(0-3)를 항상 0으로 유지하기 위한 코드
        // 게임보이에서 F 레지스터의 하위 4비트는 사용되지 않고, 항상 0이어야 함
        self.f &= 0xF0;
    }
}
