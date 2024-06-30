use nd_jpegli_sys::c_char;
use std::borrow::Cow;
use std::ptr::NonNull;

/// An error string.
#[derive(Debug)]
pub struct ErrorString {
    ptr: NonNull<c_char>,
    len: usize,
}

impl ErrorString {
    /// Create a new error string from a pointer.
    pub(crate) unsafe fn from_ptr(ptr: *mut c_char) -> Option<Self> {
        let ptr = NonNull::new(ptr)?;
        let len = unsafe {
            let mut len = 0;
            let mut ptr = ptr.as_ptr();
            while *ptr != 0 {
                ptr = ptr.add(1);
                len += 1;
            }
            len
        };

        Some(Self { ptr, len })
    }

    /// Get the length (in bytes) of this error message.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Check if this error message is empty.
    ///
    /// Note that an "ok" error message is represented by a NULL pointer,
    /// which cannot be held by this object.
    /// As a result, this should never return true.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the error string as bytes.
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr().cast(), self.len) }
    }

    /// Try to get the error string as a utf8 str.
    pub fn as_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.as_bytes())
    }

    /// Try to get the error string as a utf8 string, lossily.
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(self.as_bytes())
    }
}

impl std::fmt::Display for ErrorString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for chunk in self.as_bytes().utf8_chunks() {
            write!(f, "{}", chunk.valid())?;

            for _ in chunk.invalid() {
                write!(f, "\u{FFFD}")?;
            }
        }

        Ok(())
    }
}

impl std::error::Error for ErrorString {}

// This is just a small wrapper around malloc/free, which is threadsafe.
unsafe impl Send for ErrorString {}
