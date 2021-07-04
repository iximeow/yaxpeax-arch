use crate::StandardDecodeError;

impl From<ReadError> for StandardDecodeError {
    fn from(_: ReadError) -> StandardDecodeError {
        StandardDecodeError::ExhaustedInput
    }
}

pub enum ReadError {
    ExhaustedInput,
    IOError(&'static str),
}

pub trait Reader<Address, Item> {
    fn next(&mut self) -> Result<Item, ReadError>;
    fn next_n(&mut self, buf: &mut [Item]) -> Result<(), ReadError>;
    fn mark(&mut self);
    fn offset(&mut self) -> Address;
    fn total_offset(&mut self) -> Address;
}

pub struct U8Reader<'a> {
    start: *const u8,
    data: *const u8,
    end: *const u8,
    mark: *const u8,
    _lifetime: core::marker::PhantomData<&'a [u8]>,
}

impl<'a> U8Reader<'a> {
    pub fn new(data: &'a [u8]) -> U8Reader<'a> {
        U8Reader {
            start: data.as_ptr(),
            data: data.as_ptr(),
            end: unsafe { data.as_ptr().offset(data.len() as isize) },
            mark: data.as_ptr(),
            _lifetime: core::marker::PhantomData,
        }
    }
}

/*
#[cfg(feature = "std")]
impl<T: std::io::Read> Reader<u8> for T {
    fn next(&mut self) -> Result<u8, ReadError> {
        let mut buf = [0u8];
        match self.read(&mut buf) {
            Ok(0) => { Err(ReadError::ExhaustedInput) }
            Ok(1) => { Ok(buf[0]) }
            Err(_) => {
                Err(ReadError::IOError("error"))
            }
        }
    }
}
*/

impl Reader<u64, u8> for U8Reader<'_> {
    #[inline]
    fn next(&mut self) -> Result<u8, ReadError> {
        if self.data == self.end {
            Err(ReadError::ExhaustedInput)
        } else {
            let word = unsafe { core::ptr::read(self.data) };
            unsafe {
                self.data = self.data.offset(1);
            }
            Ok(word)
        }
    }
    #[inline]
    fn next_n(&mut self, buf: &mut [u8]) -> Result<(), ReadError> {
        if let Some(data_size) = (self.end as usize).checked_sub(self.data as usize) {
            if buf.len() > data_size {
                return Err(ReadError::ExhaustedInput);
            }
            unsafe {
                core::ptr::copy_nonoverlapping(self.data, buf.as_mut_ptr(), buf.len());
            }
            unsafe {
                self.data = self.data.offset(buf.len() as isize);
            }
            Ok(())
        } else {
            Err(ReadError::ExhaustedInput)
        }
    }
    #[inline]
    fn mark(&mut self) {
        self.mark = self.data;
    }
    #[inline]
    fn offset(&mut self) -> u64 {
        self.data as u64 - self.mark as u64
    }
    #[inline]
    fn total_offset(&mut self) -> u64 {
        self.data as u64 - self.start as u64
    }
}

impl Reader<u32, u8> for U8Reader<'_> {
    fn next(&mut self) -> Result<u8, ReadError> {
        if self.data == self.end {
            Err(ReadError::ExhaustedInput)
        } else {
            let word = unsafe { core::ptr::read(self.data) };
            unsafe {
                self.data = self.data.offset(1);
            }
            Ok(word)
        }
    }
    #[inline]
    fn next_n(&mut self, buf: &mut [u8]) -> Result<(), ReadError> {
        if let Some(data_size) = (self.end as usize).checked_sub(self.data as usize) {
            if buf.len() > data_size {
                return Err(ReadError::ExhaustedInput);
            }
            unsafe {
                core::ptr::copy_nonoverlapping(self.data, buf.as_mut_ptr(), buf.len());
            }
            self.data = unsafe { self.data.offset(buf.len() as isize) };
            Ok(())
        } else {
            Err(ReadError::ExhaustedInput)
        }
    }
    fn mark(&mut self) {
        self.mark = self.data;
    }
    fn offset(&mut self) -> u32 {
        self.data as u32 - self.mark as u32
    }
    fn total_offset(&mut self) -> u32 {
        self.data as u32 - self.start as u32
    }
}

/*
#[derive(Debug, PartialEq, Eq)]
pub struct U16le(pub u16);

impl Reader<U16le> for &[u8] {
    fn next(&mut self) -> Result<U16le, ReadError> {
        if self.len() < 2 {
            Err(ReadError::ExhaustedInput)
        } else {
            let bytes = [self[0], self[1]];
            *self = &self[2..];
            Ok(U16le(u16::from_le_bytes(bytes)))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct U16be(pub u16);

impl Reader<U16be> for &[u8] {
    fn next(&mut self) -> Result<U16be, ReadError> {
        if self.len() < 2 {
            Err(ReadError::ExhaustedInput)
        } else {
            let bytes = [self[0], self[1]];
            *self = &self[2..];
            Ok(U16be(u16::from_be_bytes(bytes)))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct U32le(pub u32);

impl Reader<U32le> for &[u8] {
    fn next(&mut self) -> Result<U32le, ReadError> {
        if self.len() < 4 {
            Err(ReadError::ExhaustedInput)
        } else {
            let bytes = [self[0], self[1], self[2], self[3]];
            *self = &self[4..];
            Ok(U32le(u32::from_le_bytes(bytes)))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct U32be(pub u32);

impl Reader<U32be> for &[u8] {
    fn next(&mut self) -> Result<U32be, ReadError> {
        if self.len() < 4 {
            Err(ReadError::ExhaustedInput)
        } else {
            let bytes = [self[0], self[1], self[2], self[3]];
            *self = &self[4..];
            Ok(U32be(u32::from_be_bytes(bytes)))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct U64le(pub u64);

impl Reader<U64le> for &[u8] {
    fn next(&mut self) -> Result<U64le, ReadError> {
        if self.len() < 8 {
            Err(ReadError::ExhaustedInput)
        } else {
            let bytes = [self[0], self[1], self[2], self[3], self[4], self[5], self[6], self[7]];
            *self = &self[8..];
            Ok(U64le(u64::from_le_bytes(bytes)))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct U64be(pub u64);

impl Reader<U64be> for &[u8] {
    fn next(&mut self) -> Result<U64be, ReadError> {
        if self.len() < 8 {
            Err(ReadError::ExhaustedInput)
        } else {
            let bytes = [self[0], self[1], self[2], self[3], self[4], self[5], self[6], self[7]];
            *self = &self[8..];
            Ok(U64be(u64::from_be_bytes(bytes)))
        }
    }
}

impl core::fmt::Display for U16le {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl core::fmt::Display for U16be {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl core::fmt::Display for U32le {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl core::fmt::Display for U32be {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl core::fmt::Display for U64le {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl core::fmt::Display for U64be {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}
*/
