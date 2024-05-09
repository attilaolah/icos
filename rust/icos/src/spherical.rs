use crate::val::Angle;

/// Normalised spherical coordinates (r = 1).
/// Uses the physics convention (ISO 80000-2:2019).
#[derive(Clone)]
pub struct Norm {
    /// Polar angle, with respect to positive polar axis "z" [0-pi].
    theta: Angle,

    /// Azimuthal angle, rotation from the initial meridian plane "xz" [0-2pi].
    phi: Angle,
}

impl Norm {
    pub fn zero() -> Self {
        Self {
            theta: 0.into(),
            phi: 0.into(),
        }
    }

    pub fn south(self, a: Angle) -> Self {
        Self {
            theta: self.theta.add(a),
            phi: self.phi,
        }
    }

    pub fn east(self, a: Angle) -> Self {
        Self {
            theta: self.theta,
            phi: self.phi.add(a),
        }
    }
}
