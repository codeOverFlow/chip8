use std::fmt;

const MEMORY_SIZE: usize = 4096;
pub const ENTRY_POINT: u16 = 0x200;
const NB_OPCODE: u16 = 35;

pub struct Cpu {
    pub memory: [u8; MEMORY_SIZE],
    pub pc: u16,
    pub v: [u8; 16],
    pub i: u16,
    pub stack: Vec<u16>,
    pub compteur_jeu: u8,
    pub compteur_son: u8,
    pub jump: Jump,
}

impl fmt::Display for Cpu {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "memory[pc]: {}\npc: {}\nv: {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}\ni: {}\nstack: {:?}\ncompteurJeu: {}\ncompteurSon: {}", 
               self.memory[self.pc as usize], 
               self.pc, 
               self.v[0], self.v[1], self.v[2], self.v[3], self.v[4], self.v[5], 
               self.v[6], self.v[7], self.v[8], self.v[9], self.v[10], self.v[11], 
               self.v[12], self.v[13], self.v[14], self.v[15], 
               self.i, 
               self.stack, 
               self.compteur_jeu,
               self.compteur_son)
    }
}

pub fn init_cpu() -> Cpu {
    return Cpu{memory: [0;MEMORY_SIZE], pc: ENTRY_POINT, v: [0;16], i: 0, stack: Vec::new(), compteur_jeu: 0, compteur_son: 0, jump: init_jump()};
}

impl Cpu {
    pub fn decompter(&mut self) {
        if self.compteur_jeu > 0 {
            self.compteur_jeu -= 1;
        }
        if self.compteur_son > 0 {
            self.compteur_son -= 1;
        }
    }

    pub fn get_op_code(&mut self) -> u16 {
        return ((self.memory[self.pc as usize] as u16) << 8) + (self.memory[(self.pc+1) as usize] as u16);
    }

    pub fn get_action(&self, opcode: u16) -> u8 {
        let mut a: u8 = 0;
        let mut res: u16;

        for action in 0..NB_OPCODE {
            res = self.jump.masque[action as usize] & opcode;

            if res == self.jump.id[action as usize] {
                a = action as u8;
                break;
            }
        }
        return a;
    } 
}

pub struct Jump {
    masque: [u16; NB_OPCODE as usize],
    id: [u16; NB_OPCODE as usize],
}

pub fn init_jump() -> Jump {
    let mut m: [u16; NB_OPCODE as usize] = [0; NB_OPCODE as usize];
    let mut i: [u16; NB_OPCODE as usize] = [0; NB_OPCODE as usize];

    m[0] = 0x0000;  i[0]=0x0FFF;          /* 0NNN */ 
    m[1] = 0xFFFF;  i[1]=0x00E0;          /* 00E0 */ 
    m[2] = 0xFFFF;  i[2]=0x00EE;          /* 00EE */ 
    m[3] = 0xF000;  i[3]=0x1000;          /* 1NNN */ 
    m[4] = 0xF000;  i[4]=0x2000;          /* 2NNN */ 
    m[5] = 0xF000;  i[5]=0x3000;          /* 3XNN */ 
    m[6] = 0xF000;  i[6]=0x4000;          /* 4XNN */ 
    m[7] = 0xF00F;  i[7]=0x5000;          /* 5XY0 */ 
    m[8] = 0xF000;  i[8]=0x6000;          /* 6XNN */ 
    m[9] = 0xF000;  i[9]=0x7000;          /* 7XNN */ 
    m[10]= 0xF00F; i[10]=0x8000;          /* 8XY0 */ 
    m[11]= 0xF00F; i[11]=0x8001;          /* 8XY1 */ 
    m[12]= 0xF00F; i[12]=0x8002;          /* 8XY2 */ 
    m[13]= 0xF00F; i[13]=0x8003;          /* BXY3 */ 
    m[14]= 0xF00F; i[14]=0x8004;          /* 8XY4 */ 
    m[15]= 0xF00F; i[15]=0x8005;          /* 8XY5 */ 
    m[16]= 0xF00F; i[16]=0x8006;          /* 8XY6 */ 
    m[17]= 0xF00F; i[17]=0x8007;          /* 8XY7 */ 
    m[18]= 0xF00F; i[18]=0x800E;          /* 8XYE */ 
    m[19]= 0xF00F; i[19]=0x9000;          /* 9XY0 */ 
    m[20]= 0xF000; i[20]=0xA000;          /* ANNN */ 
    m[21]= 0xF000; i[21]=0xB000;          /* BNNN */ 
    m[22]= 0xF000; i[22]=0xC000;          /* CXNN */ 
    m[23]= 0xF000; i[23]=0xD000;          /* DXYN */ 
    m[24]= 0xF0FF; i[24]=0xE09E;          /* EX9E */ 
    m[25]= 0xF0FF; i[25]=0xE0A1;          /* EXA1 */ 
    m[26]= 0xF0FF; i[26]=0xF007;          /* FX07 */ 
    m[27]= 0xF0FF; i[27]=0xF00A;          /* FX0A */ 
    m[28]= 0xF0FF; i[28]=0xF015;          /* FX15 */ 
    m[29]= 0xF0FF; i[29]=0xF018;          /* FX18 */ 
    m[30]= 0xF0FF; i[30]=0xF01E;          /* FX1E */ 
    m[31]= 0xF0FF; i[31]=0xF029;          /* FX29 */ 
    m[32]= 0xF0FF; i[32]=0xF033;          /* FX33 */ 
    m[33]= 0xF0FF; i[33]=0xF055;          /* FX55 */ 
    m[34]= 0xF0FF; i[34]=0xF065;          /* FX65 */ 
    return Jump{masque: m, id: i};
}
