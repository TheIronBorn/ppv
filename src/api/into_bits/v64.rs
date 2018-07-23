//! `FromBits` and `IntoBits` implementations for portable 64-bit wide vectors
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::v64::*;

impl_from_bits!(i8x8: u8x8, m8x8, i16x4, u16x4, m16x4, i32x2, u32x2, f32x2, m32x2);
impl_from_bits!(u8x8: i8x8, m8x8, i16x4, u16x4, m16x4, i32x2, u32x2, f32x2, m32x2);
impl_from_bits!(m8x8: m16x4, m32x2);

impl_from_bits!(i16x4: i8x8, u8x8, m8x8, u16x4, m16x4, i32x2, u32x2, f32x2, m32x2);
impl_from_bits!(u16x4: i8x8, u8x8, m8x8, i16x4, m16x4, i32x2, u32x2, f32x2, m32x2);
impl_from_bits!(m16x4: m32x2);

impl_from_bits!(i32x2: i8x8, u8x8, m8x8, i16x4, u16x4, m16x4, u32x2, f32x2, m32x2);
impl_from_bits!(u32x2: i8x8, u8x8, m8x8, i16x4, u16x4, m16x4, i32x2, f32x2, m32x2);
impl_from_bits!(f32x2: i8x8, u8x8, m8x8, i16x4, u16x4, m16x4, i32x2, u32x2, m32x2);
// note: m32x2 cannot be constructed from all m16x4 or m8x8 bit patterns