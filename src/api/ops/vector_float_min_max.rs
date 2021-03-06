//! Vertical (lane-wise) vector `min` and `max` for floating-point vectors.

macro_rules! impl_ops_vector_float_min_max {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident) => {
        impl $id {
            /// Minimum of two vectors.
            ///
            /// Returns a new vector containing the minimum value of each of
            /// the input vector lanes.
            #[inline]
            pub fn min(self, x: Self) -> Self {
                use codegen::llvm::simd_fmin;
                unsafe { Simd(simd_fmin(self.0, x.0)) }
            }

            /// Maximum of two vectors.
            ///
            /// Returns a new vector containing the maximum value of each of
            /// the input vector lanes.
            #[inline]
            pub fn max(self, x: Self) -> Self {
                // FIXME: https://github.com/rust-lang-nursery/stdsimd/issues/416
                // use codegen::llvm::simd_fmin;
                // unsafe { Simd(simd_fmin(self.0, x.0)) }
                let mut r = self;
                for i in 0..$id::lanes() {
                    let a = self.extract(i);
                    let b = x.extract(i);
                    r = r.replace(i, a.max(b))
                }
                r
            }
        }
        #[cfg(test)]
        interpolate_idents! {
            mod [$id _ops_vector_min_max] {
                use super::*;
                #[test]
                fn min_max() {
                    let n = $elem_ty::NAN;
                    let o = $id::splat(1. as $elem_ty);
                    let t = $id::splat(2. as $elem_ty);

                    let mut m = o; // [1., 2., 1., 2., ...]
                    let mut on = o;
                    for i in 0..$id::lanes() {
                        if i % 2 == 0 {
                            m = m.replace(i, 2. as $elem_ty);
                            on = on.replace(i, n);
                        }
                    }

                    assert_eq!(o.min(t), o);
                    assert_eq!(t.min(o), o);
                    assert_eq!(m.min(o), o);
                    assert_eq!(o.min(m), o);
                    assert_eq!(m.min(t), m);
                    assert_eq!(t.min(m), m);

                    assert_eq!(o.max(t), t);
                    assert_eq!(t.max(o), t);
                    assert_eq!(m.max(o), m);
                    assert_eq!(o.max(m), m);
                    assert_eq!(m.max(t), t);
                    assert_eq!(t.max(m), t);

                    assert_eq!(on.min(o), o);
                    assert_eq!(o.min(on), o);
                    assert_eq!(on.max(o), o);
                    assert_eq!(o.max(on), o);
                }
            }
        }
    };
}
