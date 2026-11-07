//! Readelf model — display ELF file header information.

#[derive(Debug, Clone)]
pub struct ElfHeader {
    pub class: u8,
    pub data: u8,
    pub type_: u16,
    pub machine: u16,
    pub entry: u64,
    pub phoff: u64,
    pub shoff: u64,
    pub flags: u32,
}

#[derive(Debug, Clone)]
pub struct SectionHeader {
    pub name: String,
    pub type_: u32,
    pub addr: u64,
    pub offset: u64,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct ProgramHeader {
    pub type_: u32,
    pub offset: u64,
    pub vaddr: u64,
    pub filesz: u64,
    pub memsz: u64,
}

#[derive(Debug, Clone)]
pub struct Readelf {
    pub wide_output: bool,
    pub hex_dump: bool,
}

impl Default for Readelf {
    fn default() -> Self { Self { wide_output: false, hex_dump: false } }
}

impl Readelf {
    pub fn new() -> Self { Self::default() }

    pub fn parse_header(&self, data: &[u8]) -> Option<ElfHeader> {
        if data.len() < 64 || &data[0..4] != b"\x7fELF" { return None; }
        Some(ElfHeader {
            class: data[4], data: data[5],
            type_: u16::from_le_bytes([data[16], data[17]]),
            machine: u16::from_le_bytes([data[18], data[19]]),
            entry: u64::from_le_bytes(data[24..32].try_into().ok()?),
            phoff: u64::from_le_bytes(data[32..40].try_into().ok()?),
            shoff: u64::from_le_bytes(data[40..48].try_into().ok()?),
            flags: u32::from_le_bytes(data[48..52].try_into().ok()?),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_header() {
        let r = Readelf::new();
        assert!(r.parse_header(b"not elf").is_none());
        let mut elf = vec![0u8; 64];
        elf[0..4].copy_from_slice(b"\x7fELF");
        elf[4] = 2; elf[5] = 1;
        assert!(r.parse_header(&elf).is_some());
    }
}
