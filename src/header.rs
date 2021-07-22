use crate::loader;

pub struct Header {

    //16 bytes long containing E_IDENT struct
    pub e_ident:      E_IDENT, 
    
    //details the type of object file (NONE, REL, EXEC, DYN, CORE)
    pub e_type:       u16,      

    //specifies the required architecture
    pub e_machine:    u16,

    //specifies version of ELF
    pub e_version:    u32,

    //virtual address of entry point, 0 if there is no entry point
    pub e_entry:      usize,
    
    //virtual address of program header table
    pub e_phoff:      usize,

    //virtual address of section header table
    pub e_shoff:      usize, 

    //cpu specific flags (not often used)
    pub e_flags:      u32,

    //size of elf header in bytes
    pub e_ehsize:     u16,

    //size of one entry of program header entry in bytes
    pub e_phentsize:  u16,
    
    //number of entries in the program header table
    pub e_phnum:      u16,

    //size of one entry of section header entry in bytes
    pub e_shentsize:  u16,
    
    //number of entries in the section header table
    pub e_shnum:      u16,

    //holds section header table index of the entry associated with the section name string table
    pub e_shstrndx:   u16,
}

impl Header {

    //returns uninitialized section header object
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
    
    //magic number at beginning of elf file
    //must be 7f 45 4c 46
    pub Magic:       u32,

    //32 or 64 bit elf
    pub Class:       u8,

    //specifies endianness
    pub Data:        u8,

    //specifies elf header version number
    pub Version:     u8,
    
    //identifies the operating system to which the object is targeted
    pub OS_ABI:      u8,
    
    //determines ABI version. Used to distinguish among incompatible version of an ABI
    pub ABI_Version: u8,

    //reserved and set to 0
    pub Padding:    [u8; 7],
}

impl E_IDENT {

    //creates uninitialized E_IDENT struct
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
    
    //fills the E_IDENT struct with passed values
    pub fn fill(&mut self, Magic: u32, Class: u8, Data: u8, Version: u8, OS_ABI: u8, ABI_Version: u8) {
        self.Magic       =  Magic;
        self.Class       =  Class;
        self.Data        =  Data;
        self.Version     =  Version;
        self.OS_ABI      =  OS_ABI;
        self.ABI_Version =  ABI_Version;
    }
}
