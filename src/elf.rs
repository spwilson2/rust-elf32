use core::ptr;
use core::mem;
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
            (str_section.sh_addr as usize + self.base_ptr) as *mut u8,
            str_section.sh_size as usize
        )
    }

    pub unsafe fn get_section<'b>(&'b self, section: Section) -> &'b SectionHeader {
        match section {
            Section::STRTAB => self.get_section_strtab()
        }
    }

    unsafe fn get_section_strtab<'b>(&'b self) -> &'b SectionHeader {
        let sections = self.get_sections_headers();

        &sections[self.header.e_shstrndx as usize]
    }

    //TODO: Move into the copy trait?
    pub unsafe fn copy(&self, addr: usize) {

        let mut mem_ptr = addr;

        // Copy the header.
        ptr::copy(self.base_ptr as *mut ElfHeader, addr as *mut _, 1);

        mem_ptr += mem::size_of::<ElfHeader>();

        // Section headers are now following us.
        let e_shoff = mem_ptr - addr;

        (&mut*(mem_ptr as *mut ElfHeader)).e_shoff = e_shoff as u32;

        // Copy the section headers.
        for section in self.get_sections_headers() {
            self.copy_section(section, mem_ptr);
            mem_ptr += mem::size_of::<SectionHeader>();
        }

        let str_table = self.get_str_table(self.get_sections_headers());

        // Copy the strtab.
        ptr::copy(str_table.as_ptr(), mem_ptr as *mut _, str_table.len());
    }

    unsafe fn copy_section(&self, section_header: &SectionHeader, new_addr: usize) {
        let new_ptr = new_addr as *mut SectionHeader;

        ptr::copy(section_header as *const SectionHeader, new_ptr, 1);

        // TODO: Fix for relocations that don't move the header past the data.
        let new_off = 0;

        (&mut*(new_ptr)).sh_offset = new_off;
    }
}


#[allow(non_snake_case)]
pub enum Section {
    STRTAB
}

#[repr(C)]
#[derive(Debug)]
pub struct SectionHeader {
    sh_name:        Elf32_Word,
    sh_type:        Elf32_Word,
    sh_flags:       Elf32_Word,
    sh_addr:        Elf32_Addr,
    sh_offset:      Elf32_Off,
    sh_size:        Elf32_Word,
    sh_link:        Elf32_Word,
    sh_info:        Elf32_Word,
    sh_addralign:   Elf32_Word,
    sh_entsize:     Elf32_Word,
}

impl SectionHeader {
    pub unsafe fn size(&self) -> usize {self.sh_size as usize}
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
