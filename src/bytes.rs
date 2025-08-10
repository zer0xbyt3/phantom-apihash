#[derive(Clone, Copy)]
pub struct ByteCursor <'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> ByteCursor<'a> {
    pub fn new(buf: &'a [u8]) -> Self { Self { buf, pos: 0 } }
    pub fn len(&self) -> usize { self.buf.len() }
    pub fn pos(&self) -> usize { self.pos }
    pub fn remaining(&self) -> usize { self.buf.len().saturating_sub(self.pos) }
    pub fn is_eof(&self) -> bool { self.pos >= self.buf.len() }
    pub fn seek(&mut self, pos: usize) -> Option<()> {
        if pos <= self.len() {
            self.pos = pos;
            Some(())
        } else { None }
    }
    pub fn skip(&mut self, n: usize) -> Option<()> { self.seek(self.pos + n) }
    pub fn read_u8(&mut self) -> Option<u8> {
        let b = *self.buf.get(self.pos)?;
        self.pos += 1;
        Some(b)
    }
    pub fn read_u16(&mut self) -> Option<u16> {
        let bytes: [u8; 2] = self.buf.get(self.pos..self.pos+2)?.try_into().ok()?;
        self.pos += 2;
        Some(u16::from_le_bytes(bytes))
    }
    pub fn read_u32(&mut self) -> Option<u32> {
        let bytes: [u8; 4] = self.buf.get(self.pos..self.pos+4)?.try_into().ok()?;
        self.pos += 4;
        Some(u32::from_le_bytes(bytes))
    }
    pub fn read_u64(&mut self) -> Option<u64> {
        let bytes: [u8; 8] = self.buf.get(self.pos..self.pos+8)?.try_into().ok()?;
        self.pos += 8;
        Some(u64::from_le_bytes(bytes))
    }
    pub fn read_cstring_at(&self, start: usize) -> Option<&[u8]> {
        let mut end = start;
        while end < self.buf.len() && self.buf[end] != 0 { end += 1; }
        self.buf.get(start..end)
    }
    pub fn peek_cstring_here(&self) -> Option<&[u8]> { self.read_cstring_at(self.pos) }
    pub fn read_cstring_here(&mut self) -> Option<&[u8]> {
        let start = self.pos;
        let mut end = start;
        while end < self.buf.len() && self.buf[end] != 0 { end += 1; }
        let s = self.buf.get(start..end)?;
        if end <self.buf.len() {
            self.pos = end + 1;
        } else {
            self.pos = end;
        }
        Some(s)
    }
}