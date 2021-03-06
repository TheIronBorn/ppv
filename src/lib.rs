//! Simd<[T; N]> - Portable packed vector type
//!
//! TODO:
//! - gather/scatter
//! - From
//! - grep for fixme's

#![feature(
    rust_2018_preview,
    repr_simd,
    const_fn,
    platform_intrinsics,
    stdsimd,
    aarch64_target_feature,
    arm_target_feature,
    link_llvm_intrinsics,
    core_intrinsics,
    stmt_expr_attributes,
    align_offset
)]
#![allow(non_camel_case_types, non_snake_case)]
#![cfg_attr(test, feature(plugin, hashmap_internals))]
#![cfg_attr(test, plugin(interpolate_idents))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        cast_possible_truncation,
        cast_lossless,
        cast_possible_wrap,
        cast_precision_loss
    )
)]
#![cfg_attr(
    feature = "cargo-clippy",
    deny(missing_inline_in_public_items)
)]
#![no_std]

#[macro_use]
extern crate cfg_if;

#[cfg(test)]
extern crate arrayvec;

#[allow(unused_imports)]
use core::{
    cmp, default, f32, f64, fmt, hash, hint, intrinsics, iter, marker, mem,
    ops, ptr, slice,
};

#[macro_use]
mod api;
mod codegen;
mod sealed;

#[cfg(test)]
mod test_utils;

/// Packed vector type
///
/// # Examples
///
/// ```
/// # use ppv::Simd;
/// let v = Simd::<[i32; 4]>::new(0, 1, 2, 3);
/// assert_eq!(v.extract(2), 2);
/// ```
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Simd<A: sealed::SimdArray>(
    // FIXME: this type should be private,
    // but it currently must be public for the
    // `shuffle!` macro to work: it needs to
    // access the internal `repr(simd)` type
    // to call the shuffle intrinsics.
    #[doc(hidden)] pub <A as sealed::SimdArray>::Tuple,
);

mod masks;
pub use self::masks::*;

mod v128;
pub use self::v128::*;

mod v256;
pub use self::v256::*;

// Re-export the shuffle intrinsics required by the `shuffle!` macro.
#[doc(hidden)]
pub use self::codegen::llvm::{
    __shuffle_vector16, __shuffle_vector2, __shuffle_vector32,
    __shuffle_vector4, __shuffle_vector64, __shuffle_vector8,
};

crate mod llvm {
    crate use codegen::llvm::*;
}
