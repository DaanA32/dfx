use std::num::Wrapping;

pub trait FixChecksum {
    fn checksum(self) -> Wrapping<u8>;
}

pub trait FixLength {
    fn bytes_len(self) -> u32;
}

macro_rules! impl_checksum_unsigned {
    ($type_name:ident) => {
        impl FixChecksum for $type_name {
            fn checksum(self) -> Wrapping<u8> {
                if self == 0 {
                    Wrapping('0' as u8)
                } else {
                    let mut total = Wrapping(0);
                    let mut tag = self;
                    while tag > 0 {
                        total += Wrapping((tag % 10) as u8) + Wrapping('0' as u8);
                        tag /= 10;
                    }
                    total
                }
            }
        }
    };}

macro_rules! impl_bytes_len_unsigned {
    ($type_name:ident) => {
        impl FixLength for $type_name {
            fn bytes_len(self) -> u32 {
                if self == 0 {
                    1
                } else {
                    let mut total = 0;
                    let mut tag = self;
                    while tag > 0 {
                        total += 1;
                        tag /= 10;
                    }
                    total
                }
            }
        }
    };}

impl_checksum_unsigned!(u8);
impl_checksum_unsigned!(u16);
impl_checksum_unsigned!(u32);
impl_checksum_unsigned!(u64);
impl_checksum_unsigned!(u128);
impl_bytes_len_unsigned!(u8);
impl_bytes_len_unsigned!(u16);
impl_bytes_len_unsigned!(u32);
impl_bytes_len_unsigned!(u64);
impl_bytes_len_unsigned!(u128);

macro_rules! impl_checksum_signed {
    ($type_name:ident) => {
        impl FixChecksum for $type_name {
            fn checksum(self) -> Wrapping<u8> {
                if self == 0 {
                    Wrapping('0' as u8)
                } else {
                    let checksum_neg = if self < 0 {
                        Wrapping('-' as u8)
                    } else {
                        Wrapping(0)
                    };
                    let mut total = Wrapping(0);
                    let mut tag = self.abs();
                    while tag > 0 {
                        total += Wrapping((tag % 10) as u8) + Wrapping('0' as u8);
                        tag /= 10;
                    }
                    total + checksum_neg
                }
            }
        }
    };}

macro_rules! impl_bytes_len_signed {
    ($type_name:ident) => {
        impl FixLength for $type_name {
            fn bytes_len(self) -> u32 {
                if self == 0 {
                    1
                } else {
                    let checksum_neg = if self < 0 {
                        1
                    } else {
                        0
                    };
                    let mut total = 0;
                    let mut tag = self.abs();
                    while tag > 0 {
                        total += 1;
                        tag /= 10;
                    }
                    total + checksum_neg
                }
            }
        }
    };}

impl_checksum_signed!(i8);
impl_checksum_signed!(i16);
impl_checksum_signed!(i32);
impl_checksum_signed!(i64);
impl_checksum_signed!(i128);
impl_bytes_len_signed!(i8);
impl_bytes_len_signed!(i16);
impl_bytes_len_signed!(i32);
impl_bytes_len_signed!(i64);
impl_bytes_len_signed!(i128);

impl FixChecksum for &[u8] {
    fn checksum(self) -> Wrapping<u8> {
        self.iter().map(|b| Wrapping(*b)).sum()
    }
}

impl FixLength for &[u8] {
    fn bytes_len(self) -> u32 {
        self.len() as u32
    }
}
