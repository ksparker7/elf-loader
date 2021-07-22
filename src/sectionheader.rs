use crate::header;
use crate::programheader;
use crate::loader;

#[derive(Debug)]

pub struct SectionHeader {

    //name of section, offset into the section header string table section
    pub sh_name:	    u32,

    //categorizes the sections contents and semantics
    pub sh_type:	    u32,

    //Flags that describe miscellaneous attributes
    pub sh_flags:	    usize,

    //holds address of the first byte of the section
    pub sh_addr:	    usize,

    //holds offset from beginning the file to the first byte in the section
    pub sh_offset:	    usize,

    //sections size in bytes
    pub sh_size:	    usize,

    //holds a section header table index link
    pub sh_link:	    u32,

    //holds extra information dependingon the section type
    pub sh_info:	    u32,

    //specifies byte alignment
    pub sh_addralign:   usize,

    //gives the size in bytes of fixed size strings that the section holds
    pub sh_entsize:     usize,
}

impl SectionHeader {
    pub fn new() -> Self {
        //returns an uninitialized section header struct
        Self {
            sh_name:	    0,
            sh_type:	    0,
            sh_flags:	    0,
            sh_addr:	    0,
            sh_offset:	    0,
            sh_size:	    0,
            sh_link:	    0,
            sh_info:	    0,
            sh_addralign:   0,
            sh_entsize:     0,
        }
    }
}



