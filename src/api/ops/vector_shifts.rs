//! Vertical (lane-wise) vector-vector shifts operations.

macro_rules! impl_ops_vector_shifts {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident) => {
        impl ::ops::Shl<$id> for $id {
            type Output = Self;
            #[inline]
            fn shl(self, other: Self) -> Self {
                use llvm::simd_shl;
                unsafe { Simd(simd_shl(self.0, other.0)) }
            }
        }
        impl ::ops::Shr<$id> for $id {
            type Output = Self;
            #[inline]
            fn shr(self, other: Self) -> Self {
                use llvm::simd_shr;
                unsafe { Simd(simd_shr(self.0, other.0)) }
            }
        }
        impl ::ops::ShlAssign<$id> for $id {
            #[inline]
            fn shl_assign(&mut self, other: Self) {
                *self = *self << other;
            }
        }
        impl ::ops::ShrAssign<$id> for $id {
            #[inline]
            fn shr_assign(&mut self, other: Self) {
                *self = *self >> other;
            }
        }
        #[cfg(test)]
        interpolate_idents! {
            mod [$id _ops_vector_shifts] {
                use super::*;
                #[test]
                fn ops_vector_shifts() {
                    let z = $id::splat(0 as $elem_ty);
                    let o = $id::splat(1 as $elem_ty);
                    let t = $id::splat(2 as $elem_ty);
                    let f = $id::splat(4 as $elem_ty);

                    let max =
                        $id::splat((mem::size_of::<$elem_ty>() * 8 - 1) as $elem_ty);

                    // shr
                    assert_eq!(z >> z, z);
                    assert_eq!(z >> o, z);
                    assert_eq!(z >> t, z);
                    assert_eq!(z >> t, z);

                    #[cfg(any(target_arch = "s390x", target_arch = "sparc64"))] {
                        // FIXME: rust produces bad codegen for shifts:
                        // https://github.com/rust-lang/rust/issues/52015
                        return;
                    }

                    assert_eq!(o >> z, o);
                    assert_eq!(t >> z, t);
                    assert_eq!(f >> z, f);
                    assert_eq!(f >> max, z);

                    assert_eq!(o >> o, z);
                    assert_eq!(t >> o, o);
                    assert_eq!(t >> t, z);
                    assert_eq!(f >> o, t);
                    assert_eq!(f >> t, o);
                    assert_eq!(f >> max, z);

                    // shl
                    assert_eq!(z << z, z);
                    assert_eq!(o << z, o);
                    assert_eq!(t << z, t);
                    assert_eq!(f << z, f);
                    assert_eq!(f << max, z);

                    assert_eq!(o << o, t);
                    assert_eq!(o << t, f);
                    assert_eq!(t << o, f);

                    {
                        // shr_assign
                        let mut v = o;
                        v >>= o;
                        assert_eq!(v, z);
                    }
                    {
                        // shl_assign
                        let mut v = o;
                        v <<= o;
                        assert_eq!(v, t);
                    }
                }
            }
        }
    };
}
