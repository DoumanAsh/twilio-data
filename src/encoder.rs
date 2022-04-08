use core::fmt;

pub const SEP: &str = "&";
pub const EQ: &str = "=";

#[inline]
pub fn push_pair(key: &str, value: &str, out: &mut Vec<u8>) {
    out.reserve(key.len() + value.len() + 1);

    if !out.is_empty() {
        out.extend_from_slice(SEP.as_bytes());
    }

    for part in form_urlencoded::byte_serialize(key.as_bytes()) {
        out.extend_from_slice(part.as_bytes());
    }
    out.extend_from_slice(EQ.as_bytes());
    for part in form_urlencoded::byte_serialize(value.as_bytes()) {
        out.extend_from_slice(part.as_bytes());
    }
}

#[inline]
pub fn format_pair(key: &str, value: &str, fmt: &mut fmt::Formatter) -> fmt::Result {
    for part in form_urlencoded::byte_serialize(key.as_bytes()) {
        fmt.write_str(part)?;
    }
    fmt.write_str(EQ)?;
    for part in form_urlencoded::byte_serialize(value.as_bytes()) {
        fmt.write_str(part)?;
    }

    Ok(())
}
