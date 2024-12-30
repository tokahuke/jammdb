use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    rc::Rc,
};

pub trait ToBytes<'a> {
    fn to_bytes(self) -> Bytes<'a>;
}

impl<'a> ToBytes<'a> for &'a [u8] {
    fn to_bytes(self) -> Bytes<'a> {
        Bytes::Slice(self)
    }
}

impl<'a> ToBytes<'a> for &'a str {
    fn to_bytes(self) -> Bytes<'a> {
        Bytes::Slice(self.as_bytes())
    }
}

macro_rules! byte_array_to_bytes {
    ($($n:expr),*) => (
    $(
        impl<'a> ToBytes<'a> for [u8; $n] {
            fn to_bytes(self) -> Bytes<'a> {
                Bytes::Bytes(bytes::Bytes::copy_from_slice(&self))
            }
        }
    )*
)
}

// We don't want to automatically copy arrays of any length,
// but for concenience, we'll copy arrays for integer sizes
// so that if you do i.to_be_bytes() it will work for any int.
byte_array_to_bytes!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);

impl<'a> ToBytes<'a> for String {
    fn to_bytes(self) -> Bytes<'a> {
        Bytes::String(Rc::new(self))
    }
}

impl<'a> ToBytes<'a> for Vec<u8> {
    fn to_bytes(self) -> Bytes<'a> {
        Bytes::Vec(Rc::new(self))
    }
}

impl<'a> ToBytes<'a> for bytes::Bytes {
    fn to_bytes(self) -> Bytes<'a> {
        Bytes::Bytes(self)
    }
}

impl<'a> ToBytes<'a> for &bytes::Bytes {
    fn to_bytes(self) -> Bytes<'a> {
        Bytes::Bytes(self.clone())
    }
}

impl<'a> ToBytes<'a> for Bytes<'a> {
    fn to_bytes(self) -> Bytes<'a> {
        self
    }
}

impl<'a> ToBytes<'a> for &Bytes<'a> {
    fn to_bytes(self) -> Bytes<'a> {
        self.clone()
    }
}

#[derive(Debug, Clone)]
pub enum Bytes<'a> {
    Slice(&'a [u8]),
    #[allow(clippy::enum_variant_names)]
    Bytes(bytes::Bytes),
    Vec(Rc<Vec<u8>>),
    String(Rc<String>),
}

impl Bytes<'_> {
    pub fn size(&self) -> usize {
        match self {
            Self::Slice(s) => s.len(),
            Self::Bytes(b) => b.len(),
            Self::Vec(v) => v.len(),
            Self::String(s) => s.len(),
        }
    }
}

impl AsRef<[u8]> for Bytes<'_> {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::Slice(s) => s,
            Self::Bytes(b) => b,
            Self::Vec(v) => v.as_slice(),
            Self::String(s) => s.as_bytes(),
        }
    }
}

impl Ord for Bytes<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.as_ref();
        let b = other.as_ref();
        a.cmp(b)
    }
}

impl PartialOrd for Bytes<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Bytes<'_> {
    fn eq(&self, other: &Self) -> bool {
        let a = self.as_ref();
        let b = other.as_ref();
        a.eq(b)
    }
}

impl Eq for Bytes<'_> {}

impl Hash for Bytes<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let a = self.as_ref();
        a.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_vec() {
        let vec: Vec<u8> = vec![0, 0, 0];
        let ptr = vec.as_slice()[0] as *const u8;
        let b: Bytes = vec.to_bytes();
        let ptr2 = b.as_ref()[0] as *const u8;
        assert!(ptr == ptr2);
    }

    #[test]
    fn from_str() {
        let s = "abc";
        let ptr = s.as_bytes()[0] as *const u8;
        let b: Bytes = s.to_bytes();
        let ptr2 = b.as_ref()[0] as *const u8;
        assert!(ptr == ptr2);
    }
}
