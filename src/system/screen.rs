use std::mem;
use std::ptr;
use std::fmt;


const W: u8 = 64;
const H: u8 = 32;

#[derive(Copy, Clone)]
pub struct Position {
    x: u8,
    y: u8,
}

#[derive(Copy, Clone)]
pub struct Pixel {
    position: Position,
    couleur: char,
}

#[derive(Copy, Clone)]
pub struct Screen {
    screen: [[Pixel; W as usize]; H as usize],
}


impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = String::new();
        for y in 0..H {
            for x in 0..W {
                s = s + &(self.screen[y as usize][x as usize].couleur.to_string());
            }
            s = s + "\n";
        }
        write!(f,"{}", s)
    }
}


pub fn init_screen() -> Screen {
    let mut arr: [[Pixel; W as usize]; H as usize];
    unsafe {
        arr = mem::uninitialized();
        let mut l = 0;
        for line in &mut arr[..] {
            let mut tmp: [Pixel; W as usize] = [Pixel{position: Position{x:0,y:0},couleur: '0'}; W as usize];
            for i in 0..W {
                tmp[i as usize] = Pixel{position: Position{x: i, y: l}, couleur: ' '};
            }
            ptr::write(line, tmp);
            l += 1;
        }
    }
    return Screen{screen: arr};
}

impl Screen {
    pub fn clear(&mut self) {
        for y in 0..H {
            for x in 0..W {
                self.screen[y as usize][x as usize].couleur = ' ';
            }
        }
    }
}
