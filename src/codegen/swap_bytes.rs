//! Horizontal swap bytes reductions.
//!
//! FIXME: investigate `llvm.bswap`

use crate::*;

crate trait SwapBytes {
    fn swap_bytes(self) -> Self;
}

macro_rules! impl_swap_bytes {
    (v16: $($id:ident,)+) => {
        $(
            impl SwapBytes for $id {
                #[inline]
                fn swap_bytes(self) -> Self {
                    unsafe { shuffle!(self, [1, 0]) }
                }
            }
        )+
    };
    (v32: $($id:ident,)+) => {
        $(
            impl SwapBytes for $id {
                #[inline]
                #[cfg_attr(feature = "cargo-clippy", allow(useless_transmute))]
                fn swap_bytes(self) -> Self {
                    unsafe {
                        let bytes: u8x4 = ::mem::transmute(self);
                        let result: u8x4 = shuffle!(bytes, [3, 2, 1, 0]);
                        ::mem::transmute(result)
                    }
                }
            }
        )+
    };
    (v64: $($id:ident,)+) => {
        $(
            impl SwapBytes for $id {
                #[inline]
                #[cfg_attr(feature = "cargo-clippy", allow(useless_transmute))]
                fn swap_bytes(self) -> Self {
                    unsafe {
                        let bytes: u8x8 = ::mem::transmute(self);
                        let result: u8x8 = shuffle!(bytes, [7, 6, 5, 4, 3, 2, 1, 0]);
                        ::mem::transmute(result)
                    }
                }
            }
        )+
    };
    (v128: $($id:ident,)+) => {
        $(
            impl SwapBytes for $id {
                #[inline]
                #[cfg_attr(feature = "cargo-clippy", allow(useless_transmute))]
                fn swap_bytes(self) -> Self {
                    unsafe {
                        let bytes: u8x16 = ::mem::transmute(self);
                        let result: u8x16 = shuffle!(bytes, [
                            15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0
                        ]);
                        ::mem::transmute(result)
                    }
                }
            }
        )+
    };
    (v256: $($id:ident,)+) => {
        $(
            impl SwapBytes for $id {
                #[inline]
                #[cfg_attr(feature = "cargo-clippy", allow(useless_transmute))]
                fn swap_bytes(self) -> Self {
                    unsafe {
                        let bytes: u8x32 = ::mem::transmute(self);
                        let result: u8x32 = shuffle!(bytes, [
                            31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16,
                            15, 14, 13, 12, 11, 10, 9,  8,  7,  6,  5,  4,  3,  2,  1,  0
                        ]);
                        ::mem::transmute(result)
                    }
                }
            }
        )+
    };
    (v512: $($id:ident,)+) => {
        $(
            impl SwapBytes for $id {
                #[inline]
                #[cfg_attr(feature = "cargo-clippy", allow(useless_transmute))]
                fn swap_bytes(self) -> Self {
                    unsafe {
                        let bytes: u8x64 = ::mem::transmute(self);
                        let result: u8x64 = shuffle!(bytes, [
                            63, 62, 61, 60, 59, 58, 57, 56, 55, 54, 53, 52, 51, 50, 49, 48,
                            47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 33, 32,
                            31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16,
                            15, 14, 13, 12, 11, 10, 9,  8,  7,  6,  5,  4,  3,  2,  1,  0
                        ]);
                        ::mem::transmute(result)
                    }
                }
            }
        )+
    };
}

// impl_swap_bytes!(v16: u8x2, i8x2,);
// impl_swap_bytes!(v32: u8x4, i8x4, u16x2, i16x2,);
// impl_swap_bytes!(v64: u8x8, i8x8, u16x4, i16x4, u32x2, i32x2,);
impl_swap_bytes!(
    v128: u8x16,
    i8x16,
    u16x8,
    i16x8,
    u32x4,
    i32x4,
    u64x2,
    i64x2,
    u128x1,
    i128x1,
);
impl_swap_bytes!(
    v256: u8x32,
    i8x32,
    u16x16,
    i16x16,
    u32x8,
    i32x8,
    u64x4,
    i64x4,
    u128x2,
    i128x2,
);
// impl_swap_bytes!(v512: u8x64, i8x64, u16x32, i16x32, u32x16, i32x16, u64x8, i64x8,);
