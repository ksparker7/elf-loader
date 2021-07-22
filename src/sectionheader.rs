use crate::header;
use crate::programheader;
use crate::loader;

#[derive(Debug)]

pub struct SectionHeader {
    pub sh_name:	    u32,
    pub sh_type:	    u32,
    pub sh_flags:	    usize,
    pub sh_addr:	    usize,
    pub sh_offset:	    usize,
    pub sh_size:	    usize,
    pub sh_link:	    u32,
    pub sh_info:	    u32,
    pub sh_addralign:   usize,
    pub sh_entsize:     usize,
}

impl SectionHeader {
    pub fn new() -> Self {
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



