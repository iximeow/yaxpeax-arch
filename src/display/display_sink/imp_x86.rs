//! `imp_x86` has specialized copies to append short strings to strings. buffer sizing must be
//! handled by callers, in all cases.
//!
//! the structure of all implementations here is, essentially, to take the size of the data to
//! append and execute a copy for each bit set in that size, from highest to lowest. some bits are
//! simply never checked if the input is promised to never be that large - if a string to append is
//! only 0..7 bytes long, it is sufficient to only look at the low three bits to copy all bytes.
//!
//! in this way, it is slightly more efficient to right-size which append function is used, if the
//! maximum size of input strings can be bounded well. if the maximum size of input strings cannot
//! be bounded, you shouldn't be using these functions.

/// append `data` to `buf`, assuming `data` is less than 8 bytes and that `buf` has enough space
/// remaining to hold all bytes in `data`.
///
/// Safety: callers must ensure that `buf.capacity() - buf.len() >= data.len()`.
#[inline(always)]
pub unsafe fn append_string_lt_8_unchecked(buf: &mut alloc::string::String, data: &str) {
    // Safety: we are appending only valid utf8 strings to `self.buf`, as `s` is known to
    // be valid utf8
    let buf = unsafe { buf.as_mut_vec() };
    let new_bytes = data.as_bytes();

    unsafe {
        let dest = buf.as_mut_ptr().offset(buf.len() as isize);
        let src = new_bytes.as_ptr();

        let rem = new_bytes.len() as isize;

        // set_len early because there is no way to avoid the following asm!() writing that
        // same number of bytes into buf
        buf.set_len(buf.len() + new_bytes.len());

        core::arch::asm!(
            "8:",
            "cmp {rem:e}, 4",
            "jb 9f",
            "mov {buf:e}, dword ptr [{src} + {rem} - 4]",
            "mov dword ptr [{dest} + {rem} - 4], {buf:e}",
            "sub {rem:e}, 4",
            "jz 11f",
            "9:",
            "cmp {rem:e}, 2",
            "jb 10f",
            "mov {buf:x}, word ptr [{src} + {rem} - 2]",
            "mov word ptr [{dest} + {rem} - 2], {buf:x}",
            "sub {rem:e}, 2",
            "jz 11f",
            "10:",
            "cmp {rem:e}, 1",
            "jb 11f",
            "mov {buf:l}, byte ptr [{src} + {rem} - 1]",
            "mov byte ptr [{dest} + {rem} - 1], {buf:l}",
            "11:",
            src = in(reg) src,
            dest = in(reg) dest,
            rem = inout(reg) rem => _,
            buf = out(reg) _,
            options(nostack),
        );
    }
}

/// append `data` to `buf`, assuming `data` is less than 16 bytes and that `buf` has enough space
/// remaining to hold all bytes in `data`.
///
/// Safety: callers must ensure that `buf.capacity() - buf.len() >= data.len()`.
#[inline(always)]
pub unsafe fn append_string_lt_16_unchecked(buf: &mut alloc::string::String, data: &str) {
    // Safety: we are appending only valid utf8 strings to `self.buf`, as `s` is known to
    // be valid utf8
    let buf = unsafe { buf.as_mut_vec() };
    let new_bytes = data.as_bytes();

    unsafe {
        let dest = buf.as_mut_ptr().offset(buf.len() as isize);
        let src = new_bytes.as_ptr();

        let rem = new_bytes.len() as isize;

        // set_len early because there is no way to avoid the following asm!() writing that
        // same number of bytes into buf
        buf.set_len(buf.len() + new_bytes.len());

        core::arch::asm!(
            "7:",
            "cmp {rem:e}, 8",
            "jb 8f",
            "mov {buf:r}, qword ptr [{src} + {rem} - 8]",
            "mov qword ptr [{dest} + {rem} - 8], {buf:r}",
            "sub {rem:e}, 8",
            "jz 11f",
            "8:",
            "cmp {rem:e}, 4",
            "jb 9f",
            "mov {buf:e}, dword ptr [{src} + {rem} - 4]",
            "mov dword ptr [{dest} + {rem} - 4], {buf:e}",
            "sub {rem:e}, 4",
            "jz 11f",
            "9:",
            "cmp {rem:e}, 2",
            "jb 10f",
            "mov {buf:x}, word ptr [{src} + {rem} - 2]",
            "mov word ptr [{dest} + {rem} - 2], {buf:x}",
            "sub {rem:e}, 2",
            "jz 11f",
            "10:",
            "cmp {rem:e}, 1",
            "jb 11f",
            "mov {buf:l}, byte ptr [{src} + {rem} - 1]",
            "mov byte ptr [{dest} + {rem} - 1], {buf:l}",
            "11:",
            src = in(reg) src,
            dest = in(reg) dest,
            rem = inout(reg) rem => _,
            buf = out(reg) _,
            options(nostack),
        );
    }
}

/// append `data` to `buf`, assuming `data` is less than 32 bytes and that `buf` has enough space
/// remaining to hold all bytes in `data`.
///
/// Safety: callers must ensure that `buf.capacity() - buf.len() >= data.len()`.
#[inline(always)]
pub unsafe fn append_string_lt_32_unchecked(buf: &mut alloc::string::String, data: &str) {
    // Safety: we are appending only valid utf8 strings to `self.buf`, as `s` is known to
    // be valid utf8
    let buf = unsafe { buf.as_mut_vec() };
    let new_bytes = data.as_bytes();

    unsafe {
        let dest = buf.as_mut_ptr().offset(buf.len() as isize);
        let src = new_bytes.as_ptr();

        let rem = new_bytes.len() as isize;

        // set_len early because there is no way to avoid the following asm!() writing that
        // same number of bytes into buf
        buf.set_len(buf.len() + new_bytes.len());

        core::arch::asm!(
            "6:",
            "cmp {rem:e}, 16",
            "jb 7f",
            "mov {buf:r}, qword ptr [{src} + {rem} - 16]",
            "mov qword ptr [{dest} + {rem} - 16], {buf:r}",
            "mov {buf:r}, qword ptr [{src} + {rem} - 8]",
            "mov qword ptr [{dest} + {rem} - 8], {buf:r}",
            "sub {rem:e}, 16",
            "jz 11f",
            "7:",
            "cmp {rem:e}, 8",
            "jb 8f",
            "mov {buf:r}, qword ptr [{src} + {rem} - 8]",
            "mov qword ptr [{dest} + {rem} - 8], {buf:r}",
            "sub {rem:e}, 8",
            "jz 11f",
            "8:",
            "cmp {rem:e}, 4",
            "jb 9f",
            "mov {buf:e}, dword ptr [{src} + {rem} - 4]",
            "mov dword ptr [{dest} + {rem} - 4], {buf:e}",
            "sub {rem:e}, 4",
            "jz 11f",
            "9:",
            "cmp {rem:e}, 2",
            "jb 10f",
            "mov {buf:x}, word ptr [{src} + {rem} - 2]",
            "mov word ptr [{dest} + {rem} - 2], {buf:x}",
            "sub {rem:e}, 2",
            "jz 11f",
            "10:",
            "cmp {rem:e}, 1",
            "jb 11f",
            "mov {buf:l}, byte ptr [{src} + {rem} - 1]",
            "mov byte ptr [{dest} + {rem} - 1], {buf:l}",
            "11:",
            src = in(reg) src,
            dest = in(reg) dest,
            rem = inout(reg) rem => _,
            buf = out(reg) _,
            options(nostack),
        );
    }
}
