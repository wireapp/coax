use std;
use std::fmt;
use std::io::{self, Read, Write};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpCode {
    Cont,
    Text,
    Binary,
    Close,
    Ping,
    Pong,
    Reserved
}

impl OpCode {
    fn from_u8(x: u8) -> OpCode {
        match x {
            0x0 => OpCode::Cont,
            0x1 => OpCode::Text,
            0x2 => OpCode::Binary,
            0x8 => OpCode::Close,
            0x9 => OpCode::Ping,
            0xA => OpCode::Pong,
            0x3 ... 0x7 | 0xB ... 0xF => OpCode::Reserved,
            _   => panic!("invalid opcode: {:x}", x)
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
            OpCode::Cont     => "continuation",
            OpCode::Text     => "text",
            OpCode::Binary   => "binary",
            OpCode::Close    => "close",
            OpCode::Ping     => "ping",
            OpCode::Pong     => "pong",
            OpCode::Reserved => "reserved"
        }
    }
}

#[derive(Clone, Debug)]
enum Len {
    U16(u16),
    U64(u64)
}

#[derive(Debug, Clone)]
pub struct Header {
    fst:  u8,
    snd:  u8,
    len:  Option<Len>,
    key:  Option<u32>
}

impl Header {
    pub fn empty() -> Header {
        Header {
            fst:  0,
            snd:  0,
            len:  None,
            key:  None
        }
    }

    pub fn clear(&mut self) {
        self.fst = 0;
        self.snd = 0;
        self.len = None;
        self.key = None
    }

    pub fn fin(&self) -> bool {
        self.fst & 0b1000_0000 != 0
    }

    pub fn set_fin(&mut self, yes: bool) {
        if yes {
            self.fst |= 0b1000_0000
        } else {
            self.fst &= 0b0111_1111
        }
    }

    pub fn reserved(&self) -> u8 {
        (self.fst & 0b0111_0000) >> 4
    }

    pub fn is_ctrl(&self) -> bool {
        self.fst & 0b0000_1000 != 0
    }

    pub fn is_valid(&self) -> bool {
        if self.is_ctrl() {
            if self.len() > 125 {
                return false
            }
            if !self.fin() {
                return false
            }
        }
        if self.opcode() == OpCode::Reserved {
            return false
        }
        if self.reserved() != 0 {
            return false
        }
        true
    }

    pub fn opcode(&self) -> OpCode {
        OpCode::from_u8(self.fst & 0b0000_1111)
    }

    pub fn set_opcode(&mut self, c: OpCode) {
        self.fst &= 0b1111_0000;
        match c {
            OpCode::Cont     => (),
            OpCode::Text     => self.fst |= 0b0000_0001,
            OpCode::Binary   => self.fst |= 0b0000_0010,
            OpCode::Close    => self.fst |= 0b0000_1000,
            OpCode::Ping     => self.fst |= 0b0000_1001,
            OpCode::Pong     => self.fst |= 0b0000_1010,
            OpCode::Reserved => self.fst |= 0b0000_0011 // TODO
        }
    }

    pub fn masking_key(&self) -> Option<u32> {
        self.key.clone()
    }

    pub fn masked(&self) -> bool {
        self.snd & 0b1000_0000 != 0
    }

    pub fn mask(&mut self, key: u32) {
        self.key  = Some(key);
        self.snd |= 0b1000_0000
    }

    pub fn unmask(&mut self) -> Option<u32> {
        self.key.take().map(|k| {
            self.snd &= 0b0111_1111;
            k
        })
    }

    pub fn len(&self) -> u64 {
        match self.len {
            None              => (self.snd & 0b0111_1111) as u64,
            Some(Len::U16(n)) => n as u64,
            Some(Len::U64(n)) => n
        }
    }

    pub fn set_len(&mut self, len: u64) {
        self.snd &= 0b1000_0000;
        if len < 126 {
            self.snd |= len as u8 & 0b0111_1111;
            self.len = None
        } else if len <= std::u16::MAX as u64 {
            self.snd |= 126;
            self.len = Some(Len::U16(len as u16))
        } else {
            self.snd |= 127;
            self.len = Some(Len::U64(len))
        }
        assert_eq!(len, self.len())
    }

    pub fn write<W: Write>(&self, mut w: W) -> io::Result<()> {
        w.write_u8(self.fst)?;
        w.write_u8(self.snd)?;
        match self.len {
            Some(Len::U16(n)) => {
                w.write_u16::<BigEndian>(n)?;
                self.write_key(w)
            }
            Some(Len::U64(n)) => {
                w.write_u64::<BigEndian>(n)?;
                self.write_key(w)
            }
            None => self.write_key(w)
        }
    }

    fn write_key<W: Write>(&self, mut w: W) -> io::Result<()> {
        match self.key {
            Some(k) => w.write_u32::<BigEndian>(k),
            None    => Ok(())
        }
    }

    pub fn read<R: Read>(mut r: R) -> io::Result<Header> {
        let mut h = Header::empty();
        h.fst = r.read_u8()?;
        h.snd = r.read_u8()?;
        match h.snd & 0b0111_1111 {
            126 => {
                let n = r.read_u16::<BigEndian>()?;
                h.len = Some(Len::U16(n))
            }
            127 => {
                let n = r.read_u64::<BigEndian>()?;
                h.len = Some(Len::U64(n))
            }
            _ => ()
        }
        if h.masked() {
            let k = r.read_u32::<BigEndian>()?;
            h.key = Some(k)
        }
        Ok(h)
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "opcode={:?}, reserved={:03b}, fin={}, masked={}, key={:x}, len={}",
               self.opcode(),
               self.reserved(),
               self.fin(),
               self.masked(),
               self.masking_key().unwrap_or(0),
               self.len())
    }
}

