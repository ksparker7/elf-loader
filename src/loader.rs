use std::io;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use crate::header;
use crate::programheader;
use crate::sectionheader;


//the loader loads the elf into the appropriate structs and ensures everything is correct
//the parser will later be called on the header to make sense of it
pub struct Loader {
    pub fileVec:    Vec<u8>,
    pub fileIndex:  usize,
    pub header:     header::Header,
    pub programHeaders: Vec<programheader::ProgramHeader>,
    pub sectionHeaders: Vec<sectionheader::SectionHeader>,
}

impl Loader {
    pub fn new(fileName: &str) -> Self {
        //open file and error check
        let f = match File::open(fileName) {
            Ok(b)  => b,
            Err(e) => panic!("Error reading file"),
        };
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();
        //use bufreader to copy the file into a vector byte by byte 
        match reader.read_to_end(&mut buffer){
            Ok(b)  => Self{fileVec: buffer, fileIndex: 0, header: header::Header::new(), programHeaders: vec![],sectionHeaders: vec![]},
            Err(e) => panic!("Error reading file into vector"),
        }
    }       

    pub fn load(&mut self) {
        self.loadHeader();
        self.loadProgramHeaders();
        self.loadSectionHeaders();
    }

    pub fn loadSectionHeaders(&mut self) {
        //set fileIndex to the location of the first section header
        self.fileIndex = self.header.e_shoff;

        loop {
            //break if greater than e_shnum
            if self.sectionHeaders.len() >= self.header.e_shnum as usize {
                break;
            }
            let mut sHeader = sectionheader::SectionHeader::new();
            sHeader.sh_name=self.readUInt().unwrap();
            sHeader.sh_type=self.readUInt().unwrap();
            sHeader.sh_flags=self.readUSize().unwrap();
            sHeader.sh_addr=self.readUSize().unwrap();
            sHeader.sh_offset=self.readUSize().unwrap();
            sHeader.sh_size=self.readUSize().unwrap();
            sHeader.sh_link=self.readUInt().unwrap();
            sHeader.sh_info=self.readUInt().unwrap();
            sHeader.sh_addralign=self.readUSize().unwrap();
            sHeader.sh_entsize=self.readUSize().unwrap(); 

            self.sectionHeaders.push(sHeader); 
        }
    }

    pub fn loadProgramHeaders(&mut self) {
        //set fileIndex to the location of the first program header
        self.fileIndex = self.header.e_phoff;

        loop {
            //break if greater than e_phnum
            if self.programHeaders.len() >= self.header.e_phnum as usize {
                break;
            }
            
            let mut pHeader = programheader::ProgramHeader::new();
            //set each corresponding value of the program header
            pHeader.setTYPE(self.readUInt().unwrap()); 
            if usize::BITS == 64 {pHeader.setFLAGS(self.readUInt().unwrap());}
            pHeader.setOFFSET(self.readUSize().unwrap()); 
            pHeader.setVADDR(self.readUSize().unwrap()); 
            pHeader.setPADDR(self.readUSize().unwrap()); 
            pHeader.setFILESZ(self.readUSize().unwrap()); 
            pHeader.setMEMSZ(self.readUSize().unwrap()); 
            if usize::BITS == 32 {pHeader.setFLAGS(self.readUInt().unwrap());}
            pHeader.setALIGN(self.readUSize().unwrap()); 

            //push the program vector to the array
            self.programHeaders.push(pHeader); 
        }
    }

    pub fn loadHeader(&mut self) {
        //read e_ident
        self.loadE_IDENT();
        self.header.e_type       = self.readUShort().unwrap();
        self.header.e_machine    = self.readUShort().unwrap();
        self.header.e_version    = self.readUInt().unwrap();
        self.header.e_entry      = self.readUSize().unwrap();
        self.header.e_phoff      = self.readUSize().unwrap();
        self.header.e_shoff      = self.readUSize().unwrap(); 
        self.header.e_flags      = self.readUInt().unwrap();
        self.header.e_ehsize     = self.readUShort().unwrap();
        self.header.e_phentsize  = self.readUShort().unwrap();
        self.header.e_phnum      = self.readUShort().unwrap();
        self.header.e_shentsize  = self.readUShort().unwrap();
        self.header.e_shnum      = self.readUShort().unwrap();
        self.header.e_shstrndx   = self.readUShort().unwrap();
    }
     

    //loads each value of the e_ident into the header struct
    fn loadE_IDENT(&mut self){
        self.header.e_ident.Magic       = self.readUInt().unwrap();
        self.header.e_ident.Class       = self.readUByte().unwrap();
        self.header.e_ident.Data        = self.readUByte().unwrap();
        self.header.e_ident.Version     = self.readUByte().unwrap();
        self.header.e_ident.OS_ABI      = self.readUByte().unwrap();
        self.header.e_ident.ABI_Version = self.readUByte().unwrap();
        self.fileIndex = 16;
    }
    //1 = little endian
    //2 = big endian
    pub fn readUByte(&mut self) -> Option<u8>{
        //index fileVec while ensuring we're in bounds
        let byte = self.fileVec.get(self.fileIndex)?;
        self.fileIndex+=1;
        return Some(*byte);
    }

    pub fn readUShort(&mut self) -> Option<u16> {
        match self.header.e_ident.Data {
            1   => {
                let b1 = (self.readUByte()? as u16);
                let b2 = (self.readUByte()? as u16) << 8;
                let short = b1 | b2;
                return Some(short);
            },
            2   => { 
                let b1 = (self.readUByte()? as u16) << 8;
                let b2 = self.readUByte()? as u16;
                let short = b1 | b2;
                return Some(short);
            },
            _   => {
                panic!("Undefined endianness")
            }
        }
        
    }

    pub fn readUInt(&mut self) -> Option<u32> {
        match self.header.e_ident.Data {
            1   => {
                let b1 = (self.readUShort()? as u32);
                let b2 = (self.readUShort()? as u32) << 16;
                let int = b1 | b2;
                return Some(int);
            },
            2   => { 
                let b1 = (self.readUShort()? as u32) << 16;
                let b2 = self.readUShort()? as u32;
                let int = b1 | b2;
                return Some(int);
            },
            _   => {
                panic!("Undefined endianness")
            }
        }

    }

    pub fn readULong(&mut self) -> Option<u64> {
        match self.header.e_ident.Data {
            1   => {
                let b1 = (self.readUInt()? as u64);
                let b2 = (self.readUInt()? as u64) << 32;
                let long = b1 | b2;
                return Some(long);

            },
            2   => {
                let b1 = (self.readUInt()? as u64) << 32;
                let b2 = self.readUInt()? as u64;
                let long = b1 | b2;
                return Some(long);
            },
            _   => {
                panic!("Undefined endianness")
            }
        }

    }
    pub fn readUSize(&mut self) -> Option<usize> {
        match usize::BITS {
            32 => return Some(self.readUInt().unwrap() as usize),
            _  => return Some(self.readULong().unwrap() as usize),
        } 

    }

}

