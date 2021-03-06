//! Implements portable horizontal mask reductions.

macro_rules! impl_reduction_mask {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident) => {
        impl $id {
            /// Are `all` vector lanes `true`?
            #[inline]
            pub fn all(self) -> bool {
                unsafe { crate::codegen::reductions::mask::All::all(self) }
            }
            /// Is `any` vector lane `true`?
            #[inline]
            pub fn any(self) -> bool {
                unsafe { crate::codegen::reductions::mask::Any::any(self) }
            }
            /// Are `all` vector lanes `false`?
            #[inline]
            pub fn none(self) -> bool {
                !self.any()
            }
        }

        #[cfg(test)]
        interpolate_idents! {
            mod [$id _reduction] {
                use super::*;
                #[test]
                fn all() {
                    let a = $id::splat(true);
                    assert!(a.all());
                    let a = $id::splat(false);
                    assert!(!a.all());

                    if $id::lanes() > 1 {
                        for i in 0..$id::lanes() {
                            let mut a = $id::splat(true);
                            a = a.replace(i, false);
                            assert!(!a.all());
                            let mut a = $id::splat(false);
                            a = a.replace(i, true);
                            assert!(!a.all());
                        }
                    }
                }
                #[test]
                fn any() {
                    let a = $id::splat(true);
                    assert!(a.any());
                    let a = $id::splat(false);
                    assert!(!a.any());

                    if $id::lanes() > 1 {
                        for i in 0..$id::lanes() {
                            let mut a = $id::splat(true);
                            a = a.replace(i, false);
                            assert!(a.any());
                            let mut a = $id::splat(false);
                            a = a.replace(i, true);
                            assert!(a.any());
                        }
                    }
                }
                #[test]
                fn none() {
                    let a = $id::splat(true);
                    assert!(!a.none());
                    let a = $id::splat(false);
                    assert!(a.none());

                    if $id::lanes() > 1 {
                        for i in 0..$id::lanes() {
                            let mut a = $id::splat(true);
                            a = a.replace(i, false);
                            assert!(!a.none());
                            let mut a = $id::splat(false);
                            a = a.replace(i, true);
                            assert!(!a.none());
                        }
                    }
                }
            }
        }
    };
}
