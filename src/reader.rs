use crate::StandardDecodeError;

impl From<ReadError> for StandardDecodeError {
    fn from(_: ReadError) -> StandardDecodeError {
        StandardDecodeError::ExhaustedInput
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ReadError {
    ExhaustedInput,
    IOError(&'static str),
}

/// a trait defining how `Item`-sized words are read at `Address`-positioned offsets into some
/// stream of data. for *most* uses, [`crate::U8Reader`] probably is sufficient. when
/// reading from data sources that aren't `&[u8]`, `Address` isn't a multiple of `u8`, or `Item`
/// isn't a multiple of 8 bits, `U8Reader` won't be sufficient.
pub trait Reader<Address, Item> {
    fn next(&mut self) -> Result<Item, ReadError>;
    /// read `buf`-many items from this reader in bulk. if `Reader` cannot read `buf`-many items,
    /// return `ReadError::ExhaustedInput`.
    fn next_n(&mut self, buf: &mut [Item]) -> Result<(), ReadError>;
    /// mark the current position as where to measure `offset` against.
    fn mark(&mut self);
    /// the difference, in `Address`, between the current `Reader` position and its last `mark`.
    /// when created, a `Reader`'s initial position is `mark`ed, so creating a `Reader` and
    /// immediately calling `offset()` must return `Address::zero()`.
    fn offset(&mut self) -> Address;
    /// the difference, in `Address`, between the current `Reader` position and the initial offset
    /// when constructed.
    fn total_offset(&mut self) -> Address;
}

/// a trait defining how to build a `Reader<Address, Item>` from some data source (`Self`).
/// definitions of `ReaderBuilder` are provided for `U8Reader` on `Address` and `Word` types that
/// `yaxpeax_arch` provides - external decoder implementations should also provide `ReaderBuilder`
/// impls if they use custom `Reader` types.
pub trait ReaderBuilder<Address: crate::AddressBase, Item> where Self: Sized {
    type Result: Reader<Address, Item>;

    /// construct a reader from `data` beginning at `addr` from its beginning.
    fn read_at(data: Self, addr: Address) -> Self::Result;
    /// construct a reader from `data` beginning at the start of `data`.
    fn read_from(data: Self) -> Self::Result {
        Self::read_at(data, Address::zero())
    }
}

/// a struct for `Reader` impls that can operate on units of `u8`.
pub struct U8Reader<'a> {
    start: *const u8,
    data: *const u8,
    end: *const u8,
    mark: *const u8,
    _lifetime: core::marker::PhantomData<&'a [u8]>,
}

impl<'a> U8Reader<'a> {
    pub fn new(data: &'a [u8]) -> U8Reader<'a> {

        // WHY: either on <64b systems we panic on `data.len() > isize::MAX`, or we compute end
        // without `offset` (which would be UB for such huge slices)
        #[cfg(not(target_pointer_width = "64"))]
        let end = data.as_ptr().wrapping_add(data.len());

        // SAFETY: the slice was valid, so data + data.len() does not overflow. at the moment,
        // there aren't 64-bit systems with 63 bits of virtual address space, so it's not possible
        // to have a slice length larger than 64-bit isize::MAX.
        #[cfg(target_pointer_width = "64")]
        let end = unsafe { data.as_ptr().offset(data.len() as isize) };

        U8Reader {
            start: data.as_ptr(),
            data: data.as_ptr(),
            end,
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
                (self.data as usize - self.mark as usize) as $addr_size /
                    (core::mem::size_of::<$word>() as $addr_size)
            }
            #[inline]
            fn total_offset(&mut self) -> $addr_size {
                (self.data as usize - self.start as usize) as $addr_size /
                    (core::mem::size_of::<$word>() as $addr_size)
            }
        }

        impl<'data> ReaderBuilder<$addr_size, $word> for &'data [u8] {
            type Result = U8Reader<'data>;

            fn read_at(data: Self, addr: $addr_size) -> Self::Result {
                U8Reader::new(&data[(addr as usize)..])
            }
        }
    }
}

macro_rules! u8reader_each_addr_size {
    ($word:ident, $word_from_slice:expr, $words_from_slice:expr) => {
        u8reader_reader_impl!(u64, $word, $word_from_slice, $words_from_slice);
        u8reader_reader_impl!(u32, $word, $word_from_slice, $words_from_slice);
        u8reader_reader_impl!(u16, $word, $word_from_slice, $words_from_slice);
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
