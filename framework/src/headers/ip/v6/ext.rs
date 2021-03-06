use headers::ip::v6::{Ipv6VarHeader, NextHeader};
use headers::EndOffset;
use num::FromPrimitive;
use std::default::Default;
use std::marker::PhantomData;

// All extension headers share the first two bytes, which are the next_header
// field and the header length. We can use this struct as the first section of a
// more specific header, or as a way to skip extension headers in the processing
// pipeline that we don't care about.
#[derive(Debug)]
#[repr(C, packed)]
pub struct Ipv6ExtHeader<T>
where
    T: Ipv6VarHeader,
{
    pub next_header: u8,
    pub hdr_ext_len: u8,
    pub _parent: PhantomData<T>,
}

impl<T> Default for Ipv6ExtHeader<T>
where
    T: Ipv6VarHeader,
{
    fn default() -> Ipv6ExtHeader<T> {
        Ipv6ExtHeader {
            next_header: NextHeader::NoNextHeader as u8,
            hdr_ext_len: 0,
            _parent: PhantomData,
        }
    }
}

// Generic extension headers have the next_header field.
impl<T> Ipv6VarHeader for Ipv6ExtHeader<T>
where
    T: Ipv6VarHeader,
{
    fn next_header(&self) -> Option<NextHeader> {
        FromPrimitive::from_u8(self.next_header)
    }
}

impl<T> EndOffset for Ipv6ExtHeader<T>
where
    T: Ipv6VarHeader,
{
    type PreviousHeader = T;

    #[inline]
    fn offset(&self) -> usize {
        // Hdr Ext Len: 8-bit unsigned integer, is the length of the extension
        // header in 8-octet units, not including the first 8 octets.
        (self.hdr_ext_len as usize) * 8 + 8
    }

    #[inline]
    fn size() -> usize {
        // Extension headers have two known fields of one byte each.
        2
    }

    #[inline]
    fn payload_size(&self, hint: usize) -> usize {
        // Extension headers don't include a payload length and so we use the
        // hint from the parent header, which might be another extension header
        // or the V6 header, which does include a payload length.
        hint - self.offset()
    }

    #[inline]
    fn check_correct(&self, _prev: &Self::PreviousHeader) -> bool {
        true
    }
}

impl<T> Ipv6ExtHeader<T>
where
    T: Ipv6VarHeader,
{
    pub fn new() -> Ipv6ExtHeader<T> {
        Default::default()
    }

    pub fn hdr_ext_len(&self) -> u8 {
        self.hdr_ext_len
    }

    pub fn set_hdr_ext_len(&mut self, hdr_ext_len: u8) {
        self.hdr_ext_len = hdr_ext_len;
    }

    pub fn set_next_header(&mut self, next_hdr: NextHeader) {
        self.next_header = next_hdr as u8;
    }
}
