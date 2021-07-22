
#[derive(Debug)]
pub struct ProgramHeader32 {
    pub p_type:   u32,
    pub p_offset: usize,
    pub p_vaddr:  usize,
    pub p_paddr:  usize,
    pub p_filesz: usize,
    pub p_memsz:  usize,
    pub p_flags:  u32,
    pub p_align:  usize,
}


#[derive(Debug)]
pub struct ProgramHeader64 {
    pub p_type:   u32,
    pub p_flags:  u32,
    pub p_offset: usize,
    pub p_vaddr:  usize,
    pub p_paddr:  usize,
    pub p_filesz: usize,
    pub p_memsz:  usize,
    pub p_align:  usize,
}


#[derive(Debug)]
pub enum ProgramHeader {
    ProgramHeader32(ProgramHeader32),
    ProgramHeader64(ProgramHeader64),
}


impl ProgramHeader {
    pub fn new() -> Self {
        match usize::BITS {
            32  => {
                Self::ProgramHeader32(ProgramHeader32 {
                    p_type:     0, 
                    p_offset:   0,
                    p_vaddr:    0,
                    p_paddr:    0,
                    p_filesz:   0, 
                    p_memsz:    0, 
                    p_flags:    0, 
                    p_align:    0, 
                })
            },
            64  => {
                Self::ProgramHeader64(ProgramHeader64 {
                    p_type:     0, 
                    p_flags:    0, 
                    p_offset:   0,
                    p_vaddr:    0,
                    p_paddr:    0,
                    p_filesz:   0, 
                    p_memsz:    0, 
                    p_align:    0, 
                })
            },
            _ => panic!("undefined architecture"),
        }
    }
    
    pub fn setTYPE(&mut self, val: u32) {
        match self {
            ProgramHeader::ProgramHeader32(h) => h.p_type = val,
            ProgramHeader::ProgramHeader64(h) => h.p_type = val,
        }
    }
    pub fn setFLAGS(&mut self, val: u32) {
        match self {
            ProgramHeader::ProgramHeader32(h) => h.p_flags = val,
            ProgramHeader::ProgramHeader64(h) => h.p_flags = val,
        }
    }
    pub fn setOFFSET(&mut self, val: usize) {
        match self {
            ProgramHeader::ProgramHeader32(h) => h.p_offset = val,
            ProgramHeader::ProgramHeader64(h) => h.p_offset = val,
        }
    }
    pub fn setVADDR(&mut self, val: usize) {
        match self {
            ProgramHeader::ProgramHeader32(h) => h.p_vaddr = val,
            ProgramHeader::ProgramHeader64(h) => h.p_vaddr = val,
        }
    }
    pub fn setPADDR(&mut self, val: usize) {
        match self {
            ProgramHeader::ProgramHeader32(h) => h.p_paddr = val,
            ProgramHeader::ProgramHeader64(h) => h.p_paddr = val,
        }
    }
    pub fn setFILESZ(&mut self, val: usize) {
        match self {
            ProgramHeader::ProgramHeader32(h) => h.p_filesz = val,
            ProgramHeader::ProgramHeader64(h) => h.p_filesz = val,
        }
    }
    pub fn setMEMSZ(&mut self, val: usize) {
        match self {
            ProgramHeader::ProgramHeader32(h) => h.p_memsz = val,
            ProgramHeader::ProgramHeader64(h) => h.p_memsz = val,
        }
    }
    pub fn setALIGN(&mut self, val: usize) {
        match self {
            ProgramHeader::ProgramHeader32(h) => h.p_align = val,
            ProgramHeader::ProgramHeader64(h) => h.p_align = val,
        }
    }

    pub fn getTYPE(&mut self) -> u32 {
        match self {
            ProgramHeader::ProgramHeader32(h) => h.p_type,
            ProgramHeader::ProgramHeader64(h) => h.p_type,
        }
    }
    pub fn getFLAGS(&mut self) -> u32{
        match self {
            ProgramHeader::ProgramHeader32(h) => h.p_flags,
            ProgramHeader::ProgramHeader64(h) => h.p_flags,
        }
    }
    pub fn getOFFSET(&mut self) -> usize {
        match self {
            ProgramHeader::ProgramHeader32(h) => h.p_offset,
            ProgramHeader::ProgramHeader64(h) => h.p_offset,
        }
    }
    pub fn getVADDR(&mut self) -> usize {
        match self {
            ProgramHeader::ProgramHeader32(h) => h.p_vaddr,
            ProgramHeader::ProgramHeader64(h) => h.p_vaddr,
        }
    }
    pub fn getPADDR(&mut self) -> usize {
        match self {
            ProgramHeader::ProgramHeader32(h) => h.p_paddr,
            ProgramHeader::ProgramHeader64(h) => h.p_paddr,
        }
    }
    pub fn getFILESZ(&mut self) -> usize {
        match self {
            ProgramHeader::ProgramHeader32(h) => h.p_filesz,
            ProgramHeader::ProgramHeader64(h) => h.p_filesz,
        }
    }
    pub fn getMEMSZ(&mut self) -> usize  {
        match self {
            ProgramHeader::ProgramHeader32(h) => h.p_memsz,
            ProgramHeader::ProgramHeader64(h) => h.p_memsz,
        }
    }
    pub fn getALIGN(&mut self) -> usize {
        match self {
            ProgramHeader::ProgramHeader32(h) => h.p_align,
            ProgramHeader::ProgramHeader64(h) => h.p_align,
        }
    }
}




