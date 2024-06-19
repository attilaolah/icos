use crate::val::{Angle, Val};

/// Angle at the origin between the midpoint of a face and one of its vertices.
pub fn beta() -> Angle {
    // asin(sqrt((sqrt(5) + 5) * 2 / 15) * 2 / (sqrt(5) + 1))
    Val::from(5)
        .sqrt()
        .add(&5.into())
        .mul(&2.into())
        .div(&15.into())
        .sqrt()
        .mul(&2.into())
        .div(&Val::from(5).sqrt().add(&1.into()))
        .asin()
}
