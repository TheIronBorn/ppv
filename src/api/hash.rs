//! Implements `Hash` for vector types.

macro_rules! impl_hash {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident) => {
        impl ::hash::Hash for $id {
            #[inline]
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                unsafe {
                    union A {
                        data: [$elem_ty; $id::lanes()],
                        vec: $id,
                    }
                    A { vec: *self }.data.hash(state)
                }
            }
        }

        #[cfg(test)]
        interpolate_idents! {
            mod [$id _hash] {
                use super::*;
                #[test]
                fn hash() {
                    use ::hash::{Hash, Hasher};
                    #[allow(deprecated)]
                    use ::hash::{SipHasher13};
                    type A = [$elem_ty; $id::lanes()];
                    let a: A = [42 as $elem_ty; $id::lanes()];
                    assert!(mem::size_of::<A>() == mem::size_of::<$id>());
                    #[allow(deprecated)]
                    let mut a_hash = SipHasher13::new();
                    let mut v_hash = a_hash.clone();
                    a.hash(&mut a_hash);

                    let v = $id::splat(42 as $elem_ty);
                    v.hash(&mut v_hash);
                    assert_eq!(a_hash.finish(), v_hash.finish());
                }
            }
        }
    };
}
