//! Vertical (lane-wise) vector comparisons returning vector masks.

macro_rules! impl_cmp_vertical {
    (
        [$elem_ty:ident; $elem_count:expr]:
        $id:ident,
        $mask_ty:ident,
        $is_mask:expr,($true:expr, $false:expr)
    ) => {
        impl $id {
            /// Lane-wise equality comparison.
            #[inline]
            pub fn eq(self, other: $id) -> $mask_ty {
                use llvm::simd_eq;
                Simd(unsafe { simd_eq(self.0, other.0) })
            }

            /// Lane-wise inequality comparison.
            #[inline]
            pub fn ne(self, other: $id) -> $mask_ty {
                use llvm::simd_ne;
                Simd(unsafe { simd_ne(self.0, other.0) })
            }

            /// Lane-wise less-than comparison.
            #[inline]
            pub fn lt(self, other: $id) -> $mask_ty {
                use llvm::{simd_gt, simd_lt};
                if $is_mask {
                    Simd(unsafe { simd_gt(self.0, other.0) })
                } else {
                    Simd(unsafe { simd_lt(self.0, other.0) })
                }
            }

            /// Lane-wise less-than-or-equals comparison.
            #[inline]
            pub fn le(self, other: $id) -> $mask_ty {
                use llvm::{simd_ge, simd_le};
                if $is_mask {
                    Simd(unsafe { simd_ge(self.0, other.0) })
                } else {
                    Simd(unsafe { simd_le(self.0, other.0) })
                }
            }

            /// Lane-wise greater-than comparison.
            #[inline]
            pub fn gt(self, other: $id) -> $mask_ty {
                use llvm::{simd_gt, simd_lt};
                if $is_mask {
                    Simd(unsafe { simd_lt(self.0, other.0) })
                } else {
                    Simd(unsafe { simd_gt(self.0, other.0) })
                }
            }

            /// Lane-wise greater-than-or-equals comparison.
            #[inline]
            pub fn ge(self, other: $id) -> $mask_ty {
                use llvm::{simd_ge, simd_le};
                if $is_mask {
                    Simd(unsafe { simd_le(self.0, other.0) })
                } else {
                    Simd(unsafe { simd_ge(self.0, other.0) })
                }
            }
        }
        #[cfg(test)]
        interpolate_idents! {
            mod [$id _cmp_vertical] {
                use super::*;
                #[test]
                fn cmp() {
                    let a = $id::splat($false);
                    let b = $id::splat($true);

                    let r = a.lt(b);
                    let e = $mask_ty::splat(true);
                    assert!(r == e);
                    let r = a.le(b);
                    assert!(r == e);

                    let e = $mask_ty::splat(false);
                    let r = a.gt(b);
                    assert!(r == e);
                    let r = a.ge(b);
                    assert!(r == e);
                    let r = a.eq(b);
                    assert!(r == e);

                    let mut a = a;
                    let mut b = b;
                    let mut e = e;
                    for i in 0..$id::lanes() {
                        if i % 2 == 0 {
                            a = a.replace(i, $false);
                            b = b.replace(i, $true);
                            e = e.replace(i, true);
                        } else {
                            a = a.replace(i, $true);
                            b = b.replace(i, $false);
                            e = e.replace(i, false);
                        }
                    }
                    let r = a.lt(b);
                    assert!(r == e);
                }
            }
        }
    };
}
