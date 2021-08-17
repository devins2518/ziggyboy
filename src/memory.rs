pub struct Memory {
    rom: Rom,
    vram: VRam,
    ram: Ram,
    spr_attr_table: SprAttrTable,
    io_reg: IOReg,
    hram: HRam,
    ie_reg: u8,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            rom: Rom::default(),
            vram: VRam::default(),
            ram: Ram::default(),
            spr_attr_table: SprAttrTable::default(),
            io_reg: IOReg::default(),
            hram: HRam::default(),
            ie_reg: 0x0,
        }
    }

    pub fn get_address<'a>(&'a self, addr: u16) -> &'a [u8] {
        match addr {
            0x0000..=0x3FFF => {
                #[cfg(debug_assertions)]
                println!("Attempt to read from ROM, bank 00");

                &self.rom.mem[addr as usize % self.rom.start..]
            }
            0x4000..=0x7FFF => {
                #[cfg(debug_assertions)]
                println!("Attempt to read from ROM, bank 01");

                unimplemented!()
            }
            0x8000..=0x9FFF => {
                #[cfg(debug_assertions)]
                println!("Attempt to read from VRAM");

                unimplemented!()
            }
            0xA000..=0xBFFF => {
                #[cfg(debug_assertions)]
                println!("Attempt to read from RAM");

                unimplemented!()
            }
            0xC000..=0xCFFF => {
                #[cfg(debug_assertions)]
                println!("Attempt to read from WRAM");

                unimplemented!()
            }
            0xD000..=0xDFFF => {
                #[cfg(debug_assertions)]
                println!("Attempt to read from WRAM");

                unimplemented!()
            }
            0xE000..=0xFDFF => {
                #[cfg(debug_assertions)]
                println!("Attempt to read from Echo RAM");

                unimplemented!()
            }
            0xFE00..=0xFE9F => {
                #[cfg(debug_assertions)]
                println!("Attempt to read from Sprite Attribute Table");

                unimplemented!()
            }
            0xFEA0..=0xFEFF => {
                #[cfg(debug_assertions)]
                println!("Attempt to read from prohibited area");

                unimplemented!()
            }
            0xFF00..=0xFF7F => {
                #[cfg(debug_assertions)]
                println!("Attempt to read from I/O registers");

                unimplemented!()
            }
            0xFF80..=0xFFFE => {
                #[cfg(debug_assertions)]
                println!("Attempt to read from high RAM");

                unimplemented!()
            }
            0xFFFF..=0xFFFF => {
                #[cfg(debug_assertions)]
                println!("Attempt to read from Interrupt Enable register");

                unimplemented!()
            }
        }
    }

    pub fn get_address_mut<'a>(&'a mut self, addr: u16) -> &'a mut [u8] {
        match addr {
            0x0000..=0x3FFF => {
                #[cfg(debug_assertions)]
                println!("Attempt to read from ROM, bank 00");

                &mut self.rom.mem[addr as usize % self.rom.start..]
            }
            0x4000..=0x7FFF => {
                #[cfg(debug_assertions)]
                println!("Attempt to read from ROM, bank 01");

                unimplemented!()
            }
            0x8000..=0x9FFF => {
                #[cfg(debug_assertions)]
                println!("Attempt to read from VRAM");

                unimplemented!()
            }
            0xA000..=0xBFFF => {
                #[cfg(debug_assertions)]
                println!("Attempt to read from RAM");

                unimplemented!()
            }
            0xC000..=0xCFFF => {
                #[cfg(debug_assertions)]
                println!("Attempt to read from WRAM");

                unimplemented!()
            }
            0xD000..=0xDFFF => {
                #[cfg(debug_assertions)]
                println!("Attempt to read from WRAM");

                unimplemented!()
            }
            0xE000..=0xFDFF => {
                #[cfg(debug_assertions)]
                println!("Attempt to read from Echo RAM");

                unimplemented!()
            }
            0xFE00..=0xFE9F => {
                #[cfg(debug_assertions)]
                println!("Attempt to read from Sprite Attribute Table");

                unimplemented!()
            }
            0xFEA0..=0xFEFF => {
                #[cfg(debug_assertions)]
                println!("Attempt to read from prohibited area");

                unimplemented!()
            }
            0xFF00..=0xFF7F => {
                #[cfg(debug_assertions)]
                println!("Attempt to read from I/O registers");

                unimplemented!()
            }
            0xFF80..=0xFFFE => {
                #[cfg(debug_assertions)]
                println!("Attempt to read from high RAM");

                unimplemented!()
            }
            0xFFFF..=0xFFFF => {
                #[cfg(debug_assertions)]
                println!("Attempt to read from Interrupt Enable register");

                unimplemented!()
            }
        }
    }
}

pub struct Rom {
    mem: [u8; 0x8000],
    start: usize,
}

impl Default for Rom {
    fn default() -> Self {
        let mut mem = [0; 0x8000];
        // https://gbdev.gg8.se/wiki/articles/Gameboy_Bootstrap_ROM#Contents_of_the_ROM
        mem[0x0..=0xFF].copy_from_slice(&[
            0x31, 0xfe, 0xff, 0x3e, 0x30, 0xe0, 0x00, 0xaf, 0x21, 0xff, 0x9f, 0x32, 0xcb, 0x7c,
            0x20, 0xfb, 0x21, 0x26, 0xff, 0x0e, 0x11, 0x3e, 0x80, 0x32, 0xe2, 0x0c, 0x3e, 0xf3,
            0xe2, 0x32, 0x3e, 0x77, 0x77, 0x3e, 0xfc, 0xe0, 0x47, 0x21, 0x5f, 0xc0, 0x0e, 0x08,
            0xaf, 0x32, 0x0d, 0x20, 0xfc, 0x11, 0x4f, 0x01, 0x3e, 0xfb, 0x0e, 0x06, 0xf5, 0x06,
            0x00, 0x1a, 0x1b, 0x32, 0x80, 0x47, 0x0d, 0x20, 0xf8, 0x32, 0xf1, 0x32, 0x0e, 0x0e,
            0xd6, 0x02, 0xfe, 0xef, 0x20, 0xea, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1a, 0xcd,
            0xd3, 0x00, 0xcd, 0xd4, 0x00, 0x13, 0x7b, 0xfe, 0x34, 0x20, 0xf3, 0x11, 0xe6, 0x00,
            0x06, 0x08, 0x1a, 0x13, 0x22, 0x23, 0x05, 0x20, 0xf9, 0x3e, 0x19, 0xea, 0x10, 0x99,
            0x21, 0x2f, 0x99, 0x0e, 0x0c, 0x3d, 0x28, 0x08, 0x32, 0x0d, 0x20, 0xf9, 0x2e, 0x0f,
            0x18, 0xf3, 0x3e, 0x91, 0xe0, 0x40, 0x21, 0x00, 0xc0, 0x0e, 0x00, 0x3e, 0x00, 0xe2,
            0x3e, 0x30, 0xe2, 0x06, 0x10, 0x1e, 0x08, 0x2a, 0x57, 0xcb, 0x42, 0x3e, 0x10, 0x20,
            0x02, 0x3e, 0x20, 0xe2, 0x3e, 0x30, 0xe2, 0xcb, 0x1a, 0x1d, 0x20, 0xef, 0x05, 0x20,
            0xe8, 0x3e, 0x20, 0xe2, 0x3e, 0x30, 0xe2, 0xcd, 0xc2, 0x00, 0x7d, 0xfe, 0x60, 0x20,
            0xd2, 0x0e, 0x13, 0x3e, 0xc1, 0xe2, 0x0c, 0x3e, 0x07, 0xe2, 0x18, 0x3a, 0x16, 0x04,
            0xf0, 0x44, 0xfe, 0x90, 0x20, 0xfa, 0x1e, 0x00, 0x1d, 0x20, 0xfd, 0x15, 0x20, 0xf2,
            0xc9, 0x4f, 0x06, 0x04, 0xc5, 0xcb, 0x11, 0x17, 0xc1, 0xcb, 0x11, 0x17, 0x05, 0x20,
            0xf5, 0x22, 0x23, 0x22, 0x23, 0xc9, 0x3c, 0x42, 0xb9, 0xa5, 0xb9, 0xa5, 0x42, 0x3c,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x3e, 0x01, 0xe0, 0x50,
        ]);

        Self { mem, start: 0x0 }
    }
}

pub struct VRam {
    mem: [u8; 0x8000],
    start: usize,
}

impl Default for VRam {
    fn default() -> Self {
        Self {
            mem: [0; 0x8000],
            start: 0x8000,
        }
    }
}

pub struct Ram {
    mem: [u8; 0x8000],
    start: usize,
}

impl Default for Ram {
    fn default() -> Self {
        Self {
            mem: [0; 0x8000],
            start: 0xA000,
        }
    }
}

pub struct SprAttrTable {
    mem: [u8; 0xA0],
    start: usize,
}

impl Default for SprAttrTable {
    fn default() -> Self {
        Self {
            mem: [0; 0xA0],
            start: 0xFE00,
        }
    }
}

pub struct IOReg {
    mem: [u8; 0x80],
    start: usize,
}

impl Default for IOReg {
    fn default() -> Self {
        Self {
            mem: [0; 0x80],
            start: 0xFF00,
        }
    }
}

pub struct HRam {
    mem: [u8; 0x80],
    start: usize,
}

impl Default for HRam {
    fn default() -> Self {
        Self {
            mem: [0; 0x80],
            start: 0xFF80,
        }
    }
}
