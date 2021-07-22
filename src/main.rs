#![feature(int_bits_const)]
#[warn(unused_variables)]
#[warn(non_camel_case_types)]
mod loader;
mod header;
mod programheader;
mod sectionheader;

fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*; 
    #[test]
    fn testParserReadData() {
        let mut parser = loader::Loader::new("/home/kai/Projects/elfLoader/src/binaries/ls");
        assert_eq!(0x7F, parser.readUByte().unwrap());
        assert_eq!(0x454C, parser.readUShort().unwrap());
        assert_eq!(0x46020101, parser.readUInt().unwrap());
        assert_eq!(0x0, parser.readULong().unwrap());
    }

    #[test]
    fn testReadE_IDENT() {
        let mut parser = loader::Loader::new("/home/kai/Projects/elfLoader/src/binaries/ls");
        parser.loadHeader();
        assert_eq!(0x7F454C46, parser.header.e_ident.Magic);
        assert_eq!(0x02, parser.header.e_ident.Class);
        assert_eq!(0x01, parser.header.e_ident.Data);
        assert_eq!(0x01, parser.header.e_ident.Version);
        assert_eq!(0x0, parser.header.e_ident.OS_ABI);
    }

    #[test]
    fn testHeaderLoading() {
        let mut parser = loader::Loader::new("/home/kai/Projects/elfLoader/src/binaries/ls");
        parser.load();
        assert_eq!(0x7F454C46, parser.header.e_ident.Magic);
        assert_eq!(0x02, parser.header.e_ident.Class);
        assert_eq!(0x01, parser.header.e_ident.Data);
        assert_eq!(0x01, parser.header.e_ident.Version);
        assert_eq!(0x0, parser.header.e_ident.OS_ABI);
        assert_eq!(0x3,parser.header.e_type);
        assert_eq!(0x3e,parser.header.e_machine);
        assert_eq!(0x1,parser.header.e_version);
        assert_eq!(0x5b20,parser.header.e_entry);
        assert_eq!(0x40,parser.header.e_phoff);
        assert_eq!(140208,parser.header.e_shoff);
        assert_eq!(0,parser.header.e_flags);
        assert_eq!(64,parser.header.e_ehsize);
        assert_eq!(56,parser.header.e_phentsize);
        assert_eq!(11,parser.header.e_phnum);
        assert_eq!(64,parser.header.e_shentsize);
        assert_eq!(27,parser.header.e_shnum);
        assert_eq!(26,parser.header.e_shstrndx);
    }

    #[test]
    fn testProgramHeaderLoading() {
        let mut parser = loader::Loader::new("/home/kai/Projects/elfLoader/src/binaries/ls");
        parser.load();
        let PF_X = (1<<0);
        let PF_W = (1<<1);
        let PF_R = (1<<2);
        assert_eq!(0x6, parser.programHeaders[0].getTYPE());
        assert_eq!(0x40, parser.programHeaders[0].getOFFSET());
        assert_eq!(0x40, parser.programHeaders[0].getVADDR());
        assert_eq!(0x40, parser.programHeaders[0].getPADDR());
        assert_eq!(0x268, parser.programHeaders[0].getFILESZ());
        assert_eq!(0x268, parser.programHeaders[0].getMEMSZ());
        assert_eq!(PF_R, parser.programHeaders[0].getFLAGS());
        assert_eq!(0x8, parser.programHeaders[0].getALIGN());
    }
    #[test]
    fn testSectionHeaderLoading() {
        let mut parser = loader::Loader::new("/home/kai/Projects/elfLoader/src/binaries/ls");
        parser.load();
        let PF_X = (1<<0);
        let PF_W = (1<<1);
        let PF_R = (1<<2);
        println!("{:#x?}", parser.sectionHeaders[26]);
        assert_eq!(0x222b4, parser.sectionHeaders[26].sh_offset);
    }
}
