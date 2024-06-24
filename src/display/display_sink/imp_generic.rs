/// append `data` to `buf`, assuming `data` is less than 8 bytes and that `buf` has enough space
/// remaining to hold all bytes in `data`.
///
/// Safety: callers must ensure that `buf.capacity() - buf.len() >= data.len()`.
#[inline(always)]
pub unsafe fn append_string_lt_8_unchecked(buf: &mut alloc::string::String, data: &str) {
    buf.push_str(data);
}

/// append `data` to `buf`, assuming `data` is less than 16 bytes and that `buf` has enough space
/// remaining to hold all bytes in `data`.
///
/// Safety: callers must ensure that `buf.capacity() - buf.len() >= data.len()`.
#[inline(always)]
pub unsafe fn append_string_lt_16_unchecked(buf: &mut alloc::string::String, data: &str) {
    buf.push_str(data);
}

/// append `data` to `buf`, assuming `data` is less than 32 bytes and that `buf` has enough space
/// remaining to hold all bytes in `data`.
///
/// Safety: callers must ensure that `buf.capacity() - buf.len() >= data.len()`.
#[inline(always)]
pub unsafe fn append_string_lt_32_unchecked(buf: &mut alloc::string::String, data: &str) {
    buf.push_str(data);
}
