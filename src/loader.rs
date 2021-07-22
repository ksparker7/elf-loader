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
    //individual bytes of file stored in a vec
    pub fileVec:    Vec<u8>,

    //offset into fileVec for parsing
    pub fileIndex:  usize,

    //data structure containing header information
    pub header:     header::Header,
    
    //vector of enums for each program header found in the ELF
    pub programHeaders: Vec<programheader::ProgramHeader>,

    //vector of structs for each section header found in the ELF
    pub sectionHeaders: Vec<sectionheader::SectionHeader>,
}

impl Loader {
    //initalizes a new Loader struct by loading in a fileName
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
    //loads each part of the header
    pub fn load(&mut self) {
        self.loadHeader();
        self.loadProgramHeaders();
        self.loadSectionHeaders();
    }
    
    //loads each section header into the sectionHeaders vector
    pub fn loadSectionHeaders(&mut self) {

        //set fileIndex to the location of the first section header
        self.fileIndex = self.header.e_shoff;

        //loop over each section header
        loop {
            //break if greater than e_shnum
            if self.sectionHeaders.len() >= self.header.e_shnum as usize {break;}

            //create new empty sectionHeader struct and fill each
            //it with the appropriate amount of bytes
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
            
            //add this newly formed sectionHeader to the vector of section headers
            self.sectionHeaders.push(sHeader); 
        }
    }
    
    //loads each section header into the sectionHeaders vector
    pub fn loadProgramHeaders(&mut self) {
        
        //set fileIndex to the location of the first program header
        self.fileIndex = self.header.e_phoff;

        //loop over each program header
        loop {
            
            //break if greater than e_phnum
            if self.programHeaders.len() >= self.header.e_phnum as usize {break;}

            //create new empty sectionHeader struct and fill each
            //it with the appropriate amount of bytes
            let mut pHeader = programheader::ProgramHeader::new();
            
            pHeader.setTYPE(self.readUInt().unwrap()); 
            //on 64-bit machines the setFLAGS value is the second value in the struct while in 32
            //bit machines it is later on
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
    //loads the header into self.header
    pub fn loadHeader(&mut self) {
        //read the e_ident struct
        self.loadE_IDENT();
        
        //read the rest of the header values
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
     

    //loads each value of the e_ident
    fn loadE_IDENT(&mut self){
        self.header.e_ident.Magic       = self.readUInt().unwrap();
        self.header.e_ident.Class       = self.readUByte().unwrap();
        self.header.e_ident.Data        = self.readUByte().unwrap();
        self.header.e_ident.Version     = self.readUByte().unwrap();
        self.header.e_ident.OS_ABI      = self.readUByte().unwrap();
        self.header.e_ident.ABI_Version = self.readUByte().unwrap();

        //sets file index to 16 since the rest of e_ident is padding
        self.fileIndex = 16;
    }

    //reads the next unsigned byte from the file and returns it and increments the fileIndex
    //pointer
    pub fn readUByte(&mut self) -> Option<u8>{
        //index fileVec while ensuring we're in bounds
        let byte = self.fileVec.get(self.fileIndex)?;
        self.fileIndex+=1;
        return Some(*byte);
    }

    //reads the next unsigned short from the file and returns it and increments the fileIndex
    //pointer
    //The endianness of the data is dependent on the e_ident value in the header
    //1 = return in little endian format
    //2 = return in big endian format
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

    //same as readUShort but returns unsigned int
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

    //same as readUShort but returns unsigned long
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
    //returns a UInt on 32 bit architecture
    //and a ULong on 64 bit architecture
    pub fn readUSize(&mut self) -> Option<usize> {
        match usize::BITS {
            32 => return Some(self.readUInt().unwrap() as usize),
            _  => return Some(self.readULong().unwrap() as usize),
        } 

    }

}

