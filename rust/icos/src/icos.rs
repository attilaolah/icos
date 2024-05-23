use crate::val::{sqrt, Angle, Val};

/// The golden ratio.
fn phi() -> Val {
    // (1 + sqrt(5)) / 2
    sqrt(5.into()).add(&1.into()).div(&2.into())
}

/// Angle at the origin between two vertices of an edge.
pub fn alpha() -> Angle {
    // acos(sqrt(5) / 5)
    sqrt(5.into()).div(&5.into()).acos()
}

/// Inradius:
/// radius of the inscribed squere of an icosehadron with edge length 1.
pub fn inr() -> Val {
    // phi^2 / (2 * sqrt(3))
    phi().pow(&2.into()).div(&sqrt(3.into())).div(&2.into())
}

/// Circumradius:
/// Radius of the sicrumsphere of an icosahedron with edge length 1.
pub fn cir() -> Val {
    // sqrt(phi^2 + 1) / 2
    sqrt(phi().pow(&2.into()).add(&1.into())).div(&2.into())
}

/// Midradius:
/// Radius of the midsphere of an icosahedron with edge length 1.
pub fn mid() -> Val {
    // phi / 2
    phi().div(&2.into())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::spherical::Norm;
    use approx::assert_relative_eq;
    use num_traits::ToPrimitive;
    use std::f64::consts::PI;

    #[test]
    fn test_phi() {
        assert_relative_eq!(phi().to_f64().unwrap(), 1.6180339887498950);
    }

    #[test]
    fn test_inr() {
        assert_relative_eq!(inr().to_f64().unwrap(), 0.7557613140761709);
    }

    #[test]
    fn test_cir() {
        assert_relative_eq!(cir().to_f64().unwrap(), 0.9510565162951535);
    }

    #[test]
    fn test_mid() {
        assert_relative_eq!(mid().to_f64().unwrap(), 0.8090169943749475);
    }

    #[test]
    fn test_alpha() {
        assert_relative_eq!(alpha().to_f64().unwrap() * 180.0 / PI, 63.43494882292201);
    }

    #[test]
    fn test_alpha_distance() {
        assert_relative_eq!(
            Norm::zero()
                .distance_to(Norm::zero().south(&alpha()))
                .to_f64()
                .unwrap(),
            cir().rec().to_f64().unwrap()
        );
    }
}
