use core::mem::transmute_copy;

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

impl ElfHeader {
    pub unsafe fn get_header(ptr: *mut ElfHeader) -> Self {
        transmute_copy(&*ptr)
    }
}

#[repr(C)]
struct SectionHeader {
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
