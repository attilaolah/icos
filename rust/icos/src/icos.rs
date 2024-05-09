use crate::spherical::Norm;
use crate::val::{sqrt, Angle, Val};

/// The golden ratio.
fn phi() -> Val {
    // (1 + sqrt(5)) / 2
    sqrt(5.into()).add(1.into()).div(2.into())
}

/// Angle at the origin between two vertices of an edge.
fn alpha() -> Angle {
    // let a = 1 / cir()
    // acos((a^2 - 1) / 2)
    cir().rec().pow(2.into()).sub(1.into()).div(2.into()).acos()
}

/// Inradius:
/// radius of the inscribed squere of an icosehadron with edge length 1.
pub fn inr() -> Val {
    // phi^2 / (2 * sqrt(3))
    phi().pow(2.into()).div(sqrt(3.into())).div(2.into())
}

/// Circumradius:
/// Radius of the sicrumsphere of an icosahedron with edge length 1.
pub fn cir() -> Val {
    // sqrt(phi^2 + 1) / 2
    sqrt(phi().pow(2.into()).add(1.into())).div(2.into())
}

/// Midradius:
/// Radius of the midsphere of an icosahedron with edge length 1.
pub fn mid() -> Val {
    // phi / 2
    phi().div(2.into())
}

pub fn coords() -> Vec<Norm> {
    let turn = Angle::turn();
    let half = turn.clone().div(2.into());
    let fifth = turn.clone().div(5.into());

    let top = Norm::zero();
    let row_1 = top.clone().south(alpha());
    let row_2 = row_1.clone().south(half.clone());

    vec![
        top.clone(),
        row_1.clone(),
        row_1.clone().east(fifth.clone()),
        row_1.clone().east(fifth.clone().mul(2.into())),
        row_1.clone().east(fifth.clone().mul(3.into())),
        row_1.clone().east(fifth.clone().mul(4.into())),
        row_2.clone(),
        row_2.clone().east(fifth.clone()),
        row_2.clone().east(fifth.clone().mul(2.into())),
        row_2.clone().east(fifth.clone().mul(3.into())),
        row_2.clone().east(fifth.clone().mul(4.into())),
        top.south(half),
    ]
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_relative_eq;
    use num_traits::ToPrimitive;

    #[test]
    fn test_phi() {
        assert_relative_eq!(phi().to_f64().unwrap(), 1.618033988749895);
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
    fn test_coords() {
        let c = coords();
        assert_eq!(c.len(), 12);
    }
}
