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

/* a `std::io::Read`-friendly `Reader` would take some thought. this was an old impl, and now would
 * require something like
 * ```
 * pub struct IoReader<'io, T: std::io::Read> {
 *   io: &io mut T,
 *   count: u64,
 *   start: u64,
 * }
 * ```
 */
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

macro_rules! word_wrapper {
    ($name:ident, $underlying:ident) => {
        #[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone)]
        pub struct $name(pub $underlying);

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    }
}

word_wrapper!(U16le, u16);
word_wrapper!(U16be, u16);
word_wrapper!(U32le, u32);
word_wrapper!(U32be, u32);
word_wrapper!(U64le, u64);
word_wrapper!(U64be, u64);

macro_rules! u8reader_reader_impl {
    ($addr_size:ident, $word:ident, $word_from_slice:expr, $words_from_slice:expr) => {
        impl Reader<$addr_size, $word> for U8Reader<'_> {
            #[inline]
            fn next(&mut self) -> Result<$word, ReadError> {
                let data_size = self.end as usize - self.data as usize;

                if core::mem::size_of::<$word>() > data_size {
                    return Err(ReadError::ExhaustedInput);
                }

                // `word_from_slice` knows that we have bounds-checked that `word`-many bytes are
                // available.
                let word = $word_from_slice(self.data);
                unsafe {
                    self.data = self.data.offset(core::mem::size_of::<$word>() as isize);
                }
                Ok(word)
            }
            #[inline]
            fn next_n(&mut self, buf: &mut [$word]) -> Result<(), ReadError> {
                let data_size = self.end as usize - self.data as usize;

                let words_size_bytes = buf.len() * core::mem::size_of::<$word>();
                if words_size_bytes > data_size {
                    return Err(ReadError::ExhaustedInput);
                }

                // `word_from_slice` knows that we have bounds-checked that `word`-many bytes are
                // available.
                $words_from_slice(self.data, buf);
                unsafe {
                    self.data = self.data.offset(words_size_bytes as isize);
                }
                Ok(())
            }
            #[inline]
            fn mark(&mut self) {
                self.mark = self.data;
            }
            #[inline]
            fn offset(&mut self) -> $addr_size {
                (self.data as usize - self.mark as usize) as $addr_size
            }
            #[inline]
            fn total_offset(&mut self) -> $addr_size {
                (self.data as usize - self.start as usize) as $addr_size
            }
        }

    }
}

macro_rules! u8reader_each_addr_size {
    ($word:ident, $word_from_slice:expr, $words_from_slice:expr) => {
        u8reader_reader_impl!(u64, $word, $word_from_slice, $words_from_slice);
        u8reader_reader_impl!(u32, $word, $word_from_slice, $words_from_slice);
    }
}
u8reader_each_addr_size!(u8,
    |ptr: *const u8| { unsafe { core::ptr::read(ptr) } },
    |ptr: *const u8, buf: &mut [u8]| {
        unsafe {
            core::ptr::copy_nonoverlapping(ptr, buf.as_mut_ptr(), buf.len())
        }
    }
);

u8reader_each_addr_size!(U16le,
    |ptr: *const u8| {
        let mut word = [0u8; 2];
        unsafe {
            core::ptr::copy_nonoverlapping(ptr, word.as_mut_ptr(), word.len());
        }
        U16le(u16::from_le_bytes(word))
    },
    |ptr: *const u8, buf: &mut [U16le]| {
        // `U16le` are layout-identical to u16, so we can just copy into buf
        unsafe {
            core::ptr::copy_nonoverlapping(ptr, buf.as_mut_ptr() as *mut u8, buf.len() * core::mem::size_of::<U16le>())
        }
    }
);

u8reader_each_addr_size!(U32le,
    |ptr: *const u8| {
        let mut word = [0u8; 4];
        unsafe {
            core::ptr::copy_nonoverlapping(ptr, word.as_mut_ptr(), word.len());
        }
        U32le(u32::from_le_bytes(word))
    },
    |ptr: *const u8, buf: &mut [U32le]| {
        // `U32le` are layout-identical to u32, so we can just copy into buf
        unsafe {
            core::ptr::copy_nonoverlapping(ptr, buf.as_mut_ptr() as *mut u8, buf.len() * core::mem::size_of::<U32le>())
        }
    }
);

u8reader_each_addr_size!(U64le,
    |ptr: *const u8| {
        let mut word = [0u8; 8];
        unsafe {
            core::ptr::copy_nonoverlapping(ptr, word.as_mut_ptr(), word.len());
        }
        U64le(u64::from_le_bytes(word))
    },
    |ptr: *const u8, buf: &mut [U64le]| {
        // `U64le` are layout-identical to u64, so we can just copy into buf
        unsafe {
            core::ptr::copy_nonoverlapping(ptr, buf.as_mut_ptr() as *mut u8, buf.len() * core::mem::size_of::<U64le>())
        }
    }
);

u8reader_each_addr_size!(U16be,
    |ptr: *const u8| {
        let mut word = [0u8; 2];
        unsafe {
            core::ptr::copy_nonoverlapping(ptr, word.as_mut_ptr(), word.len());
        }
        U16be(u16::from_be_bytes(word))
    },
    |ptr: *const u8, buf: &mut [U16be]| {
        // `U16be` are layout-identical to u16, so we can just copy into buf
        unsafe {
            core::ptr::copy_nonoverlapping(ptr, buf.as_mut_ptr() as *mut u8, buf.len() * core::mem::size_of::<U16be>())
        }

        // but now we have to bswap all the words
        for i in 0..buf.len() {
            buf[i] = U16be(buf[i].0.swap_bytes());
        }
    }
);

u8reader_each_addr_size!(U32be,
    |ptr: *const u8| {
        let mut word = [0u8; 4];
        unsafe {
            core::ptr::copy_nonoverlapping(ptr, word.as_mut_ptr(), word.len());
        }
        U32be(u32::from_be_bytes(word))
    },
    |ptr: *const u8, buf: &mut [U32be]| {
        // `U32be` are layout-identical to u32, so we can just copy into buf
        unsafe {
            core::ptr::copy_nonoverlapping(ptr, buf.as_mut_ptr() as *mut u8, buf.len() * core::mem::size_of::<U32be>())
        }

        // but now we have to bswap all the words
        for i in 0..buf.len() {
            buf[i] = U32be(buf[i].0.swap_bytes());
        }
    }
);

u8reader_each_addr_size!(U64be,
    |ptr: *const u8| {
        let mut word = [0u8; 8];
        unsafe {
            core::ptr::copy_nonoverlapping(ptr, word.as_mut_ptr(), word.len());
        }
        U64be(u64::from_be_bytes(word))
    },
    |ptr: *const u8, buf: &mut [U64be]| {
        // `U64be` are layout-identical to u64, so we can just copy into buf
        unsafe {
            core::ptr::copy_nonoverlapping(ptr, buf.as_mut_ptr() as *mut u8, buf.len() * core::mem::size_of::<U64be>())
        }

        // but now we have to bswap all the words
        for i in 0..buf.len() {
            buf[i] = U64be(buf[i].0.swap_bytes());
        }
    }
);
