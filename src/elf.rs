use core::ptr;
use core::slice;

type Elf32_Addr 	= u32;
type Elf32_Half 	= u16;
type Elf32_Off 		= u32;
type Elf32_Sword 	= u32;
type Elf32_Word 	= u32;
type uchar 			= u8;

const EI_NIDENT: u32 = 16;

// Arm machine type
const EM_ARM: Elf32_Half = 0x28 as Elf32_Half;

// 0x7f 'E' 'L' 'F'
const ELFMAG: u32 = 0x7f454C46;

#[repr(C)]
#[derive(Debug)]
pub struct ElfHeader {
    e_indent:       [uchar; EI_NIDENT as usize],
    e_type:         Elf32_Half,
    e_machine:      Elf32_Half,
    e_version:      Elf32_Word,
    e_entry:        Elf32_Half,
    e_phoff:        Elf32_Off,
    e_shoff:        Elf32_Off, // Offset from beginning of file to section header table.
    e_flags:        Elf32_Word,
    e_ehsize:       Elf32_Half,
    e_phentsize:    Elf32_Half,
    e_phnum:        Elf32_Half,
    e_shentsize:    Elf32_Half,  // Size of each SH entry.
    e_shnum:        Elf32_Half,  // Number of sections
    e_shstrndx:     Elf32_Half,
}

const BYTE_MASK: usize = 0xff;
fn get_byte(val: usize, byte: usize) -> u8 {
    ((val & (BYTE_MASK << byte)) >> byte) as u8
}

pub struct ElfHeadWrapper<'a> {
    pub header: &'a mut ElfHeader,
    pub base_ptr: usize,
}

impl<'a> ElfHeadWrapper<'a> {
    //pub unsafe fn new<'a>(ptr: &'a mut ElfHeader) -> &mut Self {
    pub unsafe fn new(ptr: &'a mut ElfHeader) -> Self {

        let base = ptr as *mut _ as usize;

        ElfHeadWrapper {
            header: ptr,
            base_ptr: base,
        }
    }

    pub unsafe fn test_valid(&self) -> bool {
        for i in 0..4usize {
            if !get_byte(ELFMAG as usize, i) == self.header.e_indent[i] {
                return false
            }
        }
        true
    }

    pub unsafe fn get_sections_headers<'b>(&'b self) -> &'b [SectionHeader] {
        slice::from_raw_parts(
                (self.base_ptr + self.header.e_shoff as usize) as *mut SectionHeader,
                self.header.e_shnum as usize)
    }

    pub unsafe fn get_str_table<'b>(&'b self, sections: &[SectionHeader]) -> &'b [u8] {
        let str_section = &sections[self.header.e_shstrndx as usize];

        slice::from_raw_parts(
            str_section.sh_addr as usize as *mut u8,
            str_section.sh_size as usize
        )
    }
}


#[repr(C)]
#[derive(Debug)]
pub struct SectionHeader {
    sh_name:        Elf32_Word,
    sh_type:        Elf32_Word,
    sh_flags:       Elf32_Word,
    sh_addr:        Elf32_Addr,
    sh_offfset:     Elf32_Off,
    sh_size:        Elf32_Word,
    sh_link:        Elf32_Word,
    sh_info:        Elf32_Word,
    sh_addralign:   Elf32_Word,
    sh_entsize:     Elf32_Word,
}

#[repr(u32)]
pub enum sh_type {
    NULL,
    PROGBITS,
    SYMTAB,
    STRTAB,
    RELA,
    HASH,
    DYNAMIC,
    NOTE,
    NOBITS,
    REL,
    SHLIB,
    DYNSYM,
    LOPROC = 0x70000000,
    HIPROC = 0x7fffffff,
    LOUSER = 0x80000000,
    HIUSER = 0xffffffff,
}

#[repr(u32)]
pub enum sh_flags {
    WRITE = 0x1,
    ALLOC = 0x2,
    EXECINSTR = 0x4,
    MASKPROC = 0xf0000000,
}
