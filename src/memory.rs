use crate::cartridge::Cartridge;
use log::debug;
use std::path::Path;

pub const BOOTROM_SIZE: usize = 0x0100;
pub const VRAM_SIZE: usize = 0x8000;
pub const VRAM_START: usize = 0x8000;
pub const RAM_SIZE: usize = 0x8000;
pub const RAM_START: usize = 0xC000;
pub const SPRATTRTABLE_SIZE: usize = 0xA0;
pub const SPRATTRTABLE_START: usize = 0xFE00;
pub const IOREG_SIZE: usize = 0x80;
pub const IOREG_START: usize = 0xFF00;
pub const HRAM_SIZE: usize = 0x80;
pub const HRAM_START: usize = 0xFF80;

#[derive(Clone)]
pub struct Bus {
    bootrom: [u8; BOOTROM_SIZE],
    cartridge: Cartridge,
    vram: [u8; VRAM_SIZE],
    ram: [u8; RAM_SIZE],
    spr_attr_table: [u8; SPRATTRTABLE_SIZE],
    io_reg: [u8; IOREG_SIZE],
    hram: [u8; HRAM_SIZE],
    ie_reg: u8,
}

impl Bus {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let bootrom: [u8; BOOTROM_SIZE] = [
            0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26,
            0xFF, 0x0E, 0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77,
            0x77, 0x3E, 0xFC, 0xE0, 0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95,
            0x00, 0xCD, 0x96, 0x00, 0x13, 0x7B, 0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06,
            0x08, 0x1A, 0x13, 0x22, 0x23, 0x05, 0x20, 0xF9, 0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21,
            0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20, 0xF9, 0x2E, 0x0F, 0x18,
            0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04, 0x1E, 0x02,
            0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2,
            0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64,
            0x20, 0x06, 0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xE2, 0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15,
            0x20, 0xD2, 0x05, 0x20, 0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04, 0xC5, 0xCB,
            0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17, 0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9,
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
            0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
            0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
            0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3C, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x3C,
            0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE,
            0x34, 0x20, 0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE,
            0x3E, 0x01, 0xE0, 0x50,
        ];

        Self {
            bootrom,
            cartridge: Cartridge::new(path),
            vram: [0; VRAM_SIZE],
            ram: [0; RAM_SIZE],
            spr_attr_table: [0; SPRATTRTABLE_SIZE],
            io_reg: [0; IOREG_SIZE],
            hram: [0; HRAM_SIZE],
            ie_reg: 0x0,
        }
    }

    pub fn get_bootrom(&self, addr: u16) -> u8 {
        self.bootrom[addr as usize]
    }

    pub fn get_address(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => {
                debug!("Attempt to read from ROM, bank 00: {:#06X}", addr);

                self.cartridge.get_address(addr)
            }
            0x8000..=0x9FFF => {
                debug!(
                    "Attempt to read from VRAM: {:#06X}, value: {:#04X}",
                    addr,
                    self.vram[addr as usize % VRAM_START]
                );

                self.vram[addr as usize % VRAM_START]
            }
            0xA000..=0xBFFF => {
                debug!("Attempt to read from cartridge RAM: {:#06X}", addr);

                unimplemented!()
            }
            0xC000..=0xDFFF => {
                debug!(
                    "Attempt to read from WRAM: {:#06X}, value: {:#04X}",
                    addr,
                    self.ram[addr as usize % RAM_START]
                );

                self.ram[addr as usize % RAM_START]
            }
            0xE000..=0xFDFF => {
                debug!(
                    "Attempt to read from Echo RAM: {:#06X}, value: {:#04X}",
                    addr,
                    self.ram[(addr as usize - 0x2000) % RAM_START]
                );

                // I think
                // 0xE000 - 0xC000 = 0x2000
                self.ram[(addr as usize - 0x2000) % RAM_START]
            }
            0xFE00..=0xFE9F => {
                debug!(
                    "Attempt to read from Sprite Attribute Table: {:#06X}, value: {:#04X}",
                    addr,
                    self.spr_attr_table[addr as usize % SPRATTRTABLE_START]
                );

                self.spr_attr_table[addr as usize % SPRATTRTABLE_START]
            }
            0xFEA0..=0xFEFF => {
                debug!("Attempt to read from prohibited area: {:#06X}", addr);

                0x00
            }
            0xFF00..=0xFF7F => {
                debug!(
                    "Attempt to read from I/O registers: {:#06X}, value: {:#04X}",
                    addr,
                    self.io_reg[addr as usize % IOREG_START]
                );

                self.io_reg[addr as usize % IOREG_START]
            }
            0xFF80..=0xFFFE => {
                debug!(
                    "Attempt to read from high RAM: {:#06X}, value: {:#04X}",
                    addr,
                    self.hram[addr as usize % HRAM_START]
                );

                self.hram[addr as usize % HRAM_START]
            }
            0xFFFF => {
                debug!(
                    "Attempt to read from Interrupt Enable register: {:#06X}, value: {:#04X}",
                    addr, self.ie_reg
                );

                self.ie_reg
            }
        }
    }

    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        match addr {
            0x0000..=0x7FFF => {
                debug!("Attempt to write to ROM, bank 00. {:#06X}", addr);

                self.cartridge.write_address(addr, byte);
            }
            0x8000..=0x9FFF => {
                debug!("Attempt to write to VRAM. Addr: {:#06X}", addr);

                self.vram[addr as usize % VRAM_START] = byte;
            }
            0xA000..=0xBFFF => {
                debug!("Attempt to write to RAM. Addr: {:#06X}", addr);

                unimplemented!()
            }
            0xC000..=0xDFFF => {
                debug!("Attempt to write to WRAM. Addr: {:#06X}", addr);

                self.ram[addr as usize % RAM_START] = byte;
            }
            0xE000..=0xFDFF => {
                debug!("Attempt to write to Echo RAM. Addr: {:#06X}", addr);

                unimplemented!()
            }
            0xFE00..=0xFE9F => {
                debug!(
                    "Attempt to write to Sprite Attribute Table. Addr: {:#06X}",
                    addr
                );

                self.spr_attr_table[addr as usize % SPRATTRTABLE_START] = byte;
            }
            0xFEA0..=0xFEFF => {
                debug!("Attempt to write to prohibited area: {:#06X}", addr);
            }
            0xFF00..=0xFF7F => {
                debug!("Attempt to write to I/O registers. Addr: {:#06X}", addr);

                self.io_reg[addr as usize % IOREG_START] = byte;
            }
            0xFF80..=0xFFFE => {
                debug!("Attempt to write to high RAM. Addr: {:#06X}", addr);

                self.hram[addr as usize % HRAM_START] = byte;
            }
            0xFFFF..=0xFFFF => {
                debug!(
                    "Attempt to write to Interrupt Enable register. Addr: {:#06X}",
                    addr
                );

                self.ie_reg = byte;
            }
        }
    }
}
