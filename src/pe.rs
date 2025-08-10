use crate::bytes; 

#[derive(Debug)]
pub struct DosHeader {
    pub e_magic: u16,
    pub e_lfanew: u32
}

#[derive(Debug)]
pub enum ParseError {
    Truncated(&'static str),
    BadMagic(&'static str),
    OutOfRange(&'static str),
}

type Res<T> = Result<T, ParseError>;

const DOS_MAGIC: u16 = 0x5A4D;
const PE_MAGIC:  u32 = 0x0000_4550;
const OFF_E_LFANEW: usize = 0x3C;

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
    pub fn parse_dos_header(&self) -> Res<DosHeader> {
        let mut c = self.cursor;

        // e_magic @ 0x00
        c.seek(0).ok_or(ParseError::Truncated("truncated before e_magic"))?;
        let e_magic = c.read_u16().ok_or(ParseError::Truncated("cannot read e_magic"))?;
        if e_magic != DOS_MAGIC { // 'MZ'
            return Err(ParseError::BadMagic("bad MZ signature"));
        }

        // e_lfanew @ 0x3C
        c.seek(OFF_E_LFANEW).ok_or(ParseError::Truncated("truncated before e_lfanew"))?;
        let e_lfanew = c.read_u32().ok_or(ParseError::Truncated("cannot read e_lfanew"))?;

        Ok(DosHeader { e_magic, e_lfanew })
    }
    pub fn parse_pe_signature(&self) -> Res<(u32, u32)> {
        let dos = self.parse_dos_header()?;
        let off = usize::try_from(dos.e_lfanew)
            .map_err(|_| ParseError::OutOfRange("e_lfanew > usize"))?;

        let mut c = self.cursor;
        c.seek(off).ok_or(ParseError::Truncated("seek PE signature"))?;
        let sig = c.read_u32().ok_or(ParseError::Truncated("read PE signature"))?;
        if sig != PE_MAGIC {
            return Err(ParseError::BadMagic("PE\\0\\0"));
        }

        Ok((sig, dos.e_lfanew))
    }
}