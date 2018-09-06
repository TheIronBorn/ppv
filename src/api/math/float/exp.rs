//! Implements vertical (lane-wise) floating-point `exp`.

macro_rules! impl_math_float_exp {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident | $test_tt:tt) => {
        impl $id {
            /// Returns the exponential function of `self`: `e^(self)`.
            #[inline]
            pub fn exp(self) -> Self {
                use crate::codegen::math::float::exp::Exp;
                Exp::exp(self)
            }
        }

        test_if!{
            $test_tt:
            interpolate_idents! {
                pub mod [$id _math_exp] {
                    use super::*;
                    #[cfg_attr(not(target_arch = "wasm32"), test)] #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
                    fn exp() {
                        let z = $id::splat(0 as $elem_ty);
                        let o = $id::splat(1 as $elem_ty);
                        assert_eq!(o, z.exp());

                        let e = $id::splat(::core::f64::consts::E as $elem_ty);
                        assert_eq!(e, o.exp());
                    }
                }
            }
        }
    };
}