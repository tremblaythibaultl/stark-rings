use ark_ff::Field;
use num_bigint::BigUint;

use super::Pow2Rp64Config;
use crate::{
    cyclotomic_ring::{models::pow2_debug::Fp64Pow2, CyclotomicPolyRingGeneral},
    traits::{WithL2Norm, WithLinfNorm},
    OverField, PolyRing, WithRot,
};

pub type Pow2CyclotomicPolyRing<const Q: u64, const PHI_D: usize> =
    CyclotomicPolyRingGeneral<Pow2Rp64Config<Q, PHI_D>, 1, PHI_D>;

impl<const Q: u64, const PHI_D: usize> WithL2Norm for Pow2CyclotomicPolyRing<Q, PHI_D> {
    fn l2_norm_squared(&self) -> BigUint {
        self.coeffs().l2_norm_squared()
    }
}

impl<const Q: u64, const PHI_D: usize> WithLinfNorm for Pow2CyclotomicPolyRing<Q, PHI_D> {
    fn linf_norm(&self) -> BigUint {
        self.coeffs().linf_norm()
    }
}

impl<const Q: u64, const PHI_D: usize> WithRot for Pow2CyclotomicPolyRing<Q, PHI_D> {
    fn multiply_by_xi(&self, i: usize) -> Vec<Self::BaseRing> {
        let bs = self.0;
        #[cfg(not(feature = "native-array"))]
        let len = bs.ncols();
        #[cfg(feature = "native-array")]
        let len = bs.len();
        assert_eq!(len, PHI_D);
        let mut result = vec![<Fp64Pow2<Q, PHI_D> as Field>::ZERO; len];
        for (j, &coeff) in bs.iter().enumerate() {
            if j + i < len {
                result[(j + i) % len] += coeff;
            } else {
                result[(j + i) % len] -= coeff;
            }
        }
        result
    }
}

impl<const Q: u64, const PHI_D: usize> OverField for Pow2CyclotomicPolyRing<Q, PHI_D> {}
