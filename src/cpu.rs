use rand::Rng;

#[derive(Debug)]
pub struct Cpu {
    v: [u8; 16],
    stack: [u16; 32],
    ram: [u8; 4096],
    i: usize,  // 16 bits
    pc: usize, // 16 bits
    sp: usize,
    dt: u8,
    st: u8,
}

impl Default for Cpu {
    fn default() -> Self {
        let mut ram = [0; 4096];
        let fonts = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        ram[..0x80].copy_from_slice(&fonts);
        Self {
            v: [0; 16],
            stack: [0; 32],
            ram,
            i: 0,
            pc: 0,
            sp: 0,
            dt: 0,
            st: 0,
        }
    }
}

impl Cpu {
    pub fn decode_opcode(&mut self, opcode: u16) {
        let major = ((opcode & 0xF) >> 12) as u8;
        let minor = opcode & 0x000F;

        match major {
            0 => match opcode {
                0x00E0 => todo!("clear display"),
                0x00EE => {
                    self.pc = self.stack[self.sp] as usize;
                    self.sp = self.sp.saturating_sub(1);
                }
                _ => unreachable!("No opcode at this value."),
            },
            1 => self.pc = (opcode & 0x0FFF) as usize,
            2 => {
                self.sp += 1;
                self.stack[self.sp] = self.pc as u16;
                self.pc = (opcode & 0x0FFF) as usize
            }
            3 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let kk = (opcode & 0x00FF) as u8;

                if self.v[x] == kk {
                    self.pc += 2;
                }
            }
            4 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let kk = (opcode & 0x00FF) as u8;

                if self.v[x] != kk {
                    self.pc += 2;
                }
            }
            5 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;

                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
            }
            6 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let kk = (opcode & 0x00FF) as u8;

                self.v[x] = kk;
            }
            7 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let kk = (opcode & 0x00FF) as u8;

                self.v[x] = self.v[x] + kk;
            }
            8 => match minor {
                0 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let y = ((opcode & 0x00F0) >> 4) as usize;

                    self.v[x] = self.v[y];
                }
                1 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let y = ((opcode & 0x00F0) >> 4) as usize;

                    self.v[x] = self.v[x] | self.v[y]
                }
                2 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let y = ((opcode & 0x00F0) >> 4) as usize;

                    self.v[x] = self.v[x] & self.v[y]
                }
                3 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let y = ((opcode & 0x00F0) >> 4) as usize;

                    self.v[x] = self.v[x] ^ self.v[y]
                }
                4 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let y = ((opcode & 0x00F0) >> 4) as usize;

                    let new_value = self.v[x] as u16 + self.v[y] as u16;
                    self.v[0xF] = (new_value > 255) as u8;
                    self.v[x] = new_value as u8;
                }
                5 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let y = ((opcode & 0x00F0) >> 4) as usize;
                    self.v[0xF] = (self.v[x] > self.v[y]) as u8;
                    let new_value = self.v[x].wrapping_sub(self.v[y]);
                    self.v[x] = new_value;
                }
                6 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    self.v[0xF] = (self.v[x] & 0x1 == 1) as u8;
                    self.v[x] >>= 1;
                }
                7 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let y = ((opcode & 0x00F0) >> 4) as usize;
                    self.v[0xF] = (self.v[x] < self.v[y]) as u8;
                    let new_value = self.v[x].wrapping_sub(self.v[y]);
                    self.v[x] = new_value;
                }
                0xE => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    self.v[0xF] = (self.v[x] & 0x80 == 1) as u8;
                    self.v[x] <<= 1;
                }
            },
            9 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;

                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            }
            0xA => {
                self.i = (opcode & 0x0FFF) as usize;
            }
            0xB => {
                self.pc = (opcode & 0x0FFF) as usize + self.v[0] as usize;
            }
            0xC => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let kk = (opcode & 0x00FF) as u8;
                let random: u8 = rand::thread_rng().gen();
                self.v[x] = random & kk;
            }
            _ => unreachable!("No opcode at this value"),
        }
    }
}
