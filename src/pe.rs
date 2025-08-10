use crate::bytes; 

#[derive(Debug)]
pub struct DosHeader {
    pub e_magic: u16,
    pub e_lfanew: u32
}

pub struct PEParser<'a> {
    cursor: bytes::ByteCursor<'a>
}

impl<'a> PEParser <'a>{
    pub fn new(buf: &'a [u8]) -> Self { 
        let cursor = bytes::ByteCursor::new(buf);
        Self { cursor }
    }
    pub fn cursor(&self) -> &bytes::ByteCursor<'a> { &self.cursor }
    pub fn cursor_mut(&mut self) -> &mut bytes::ByteCursor<'a> { &mut self.cursor }
    pub fn parse_dos_header(&self) -> Result<DosHeader, &'static str> {
        let mut c = self.cursor;

        // e_magic @ 0x00
        c.seek(0).ok_or("truncated before e_magic")?;
        let e_magic = c.read_u16().ok_or("cannot read e_magic")?;
        if e_magic != 0x5A4D { // 'MZ'
            return Err("bad MZ signature");
        }

        // e_lfanew @ 0x3C
        c.seek(0x3c).ok_or("truncated before e_lfanew")?;
        let e_lfanew = c.read_u32().ok_or("cannot read e_lfanew")?;

        Ok(DosHeader { e_magic, e_lfanew })
    }
}