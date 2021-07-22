use crate::loader;

pub struct Header {
      pub e_ident:      E_IDENT, 
      pub e_type:       u16,      
      pub e_machine:    u16,
      pub e_version:    u32,
      pub e_entry:      usize,
      pub e_phoff:      usize,
      pub e_shoff:      usize, 
      pub e_flags:      u32,
      pub e_ehsize:     u16,
      pub e_phentsize:  u16,
      pub e_phnum:      u16,
      pub e_shentsize:  u16,
      pub e_shnum:      u16,
      pub e_shstrndx:   u16,
}

impl Header {
    pub fn new() -> Self {
        Self {
              e_ident:      E_IDENT::new(), 
              e_type:       0,      
              e_machine:    0,
              e_version:    0,
              e_entry:      0,
              e_phoff:      0,
              e_shoff:      0, 
              e_flags:      0,
              e_ehsize:     0,
              e_phentsize:  0,
              e_phnum:      0,
              e_shentsize:  0,
              e_shnum:      0,
              e_shstrndx:   0,
        }
    }
}


pub struct E_IDENT {
    pub Magic:       u32,
    pub Class:       u8,
    pub Data:        u8,
    pub Version:     u8,
    pub OS_ABI:      u8,
    pub ABI_Version: u8,
    pub Padding:    [u8; 7],
}

impl E_IDENT {
    pub fn new() -> Self {
        Self {
            Magic:       0,
            Class:       0,
            Data:        2,
            Version:     0,
            OS_ABI:      0,
            ABI_Version: 0,
            Padding: [0; 7],
        }
    }

    pub fn fill(&mut self, Magic: u32, Class: u8, Data: u8, Version: u8, OS_ABI: u8, ABI_Version: u8) {
        self.Magic       =  Magic;
        self.Class       =  Class;
        self.Data        =  Data;
        self.Version     =  Version;
        self.OS_ABI      =  OS_ABI;
        self.ABI_Version =  ABI_Version;
    }
}
