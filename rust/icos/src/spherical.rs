use crate::val::{Angle, Val};

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
            theta: Angle::zero(),
            phi: Angle::zero(),
        }
    }

    pub fn north(self, a: &Angle) -> Self {
        self.south(&a.clone().mul(&(-1).into()))
    }

    pub fn south(self, a: &Angle) -> Self {
        Self {
            theta: self.theta.add(a),
            phi: self.phi,
        }
    }

    pub fn east(self, a: &Angle) -> Self {
        Self {
            theta: self.theta,
            phi: self.phi.add(a),
        }
    }

    pub fn west(self, a: &Angle) -> Self {
        self.east(&a.clone().mul(&(-1).into()))
    }

    pub fn distance_to(self, to: Self) -> Val {
        let dx = to.x().sub(&self.x()).pow(&2.into());
        let dy = to.y().sub(&self.y()).pow(&2.into());
        let dz = to.z().sub(&self.z()).pow(&2.into());
        dx.add(&dy).add(&dz).sqrt()
    }

    pub fn x(&self) -> Val {
        self.theta.clone().sin().mul(&self.phi.clone().cos())
    }

    pub fn y(&self) -> Val {
        self.theta.clone().sin().mul(&self.phi.clone().sin())
    }

    pub fn z(&self) -> Val {
        self.theta.clone().cos()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::val::sqrt;
    use approx::assert_relative_eq;
    use num_traits::ToPrimitive;

    #[test]
    fn test_distance_to() {
        let half_turn = Angle::turn().div(&2.into());
        let quarter_turn = half_turn.clone().div(&2.into());

        // Zero distance.
        assert_relative_eq!(
            Norm::zero().distance_to(Norm::zero()).to_f64().unwrap(),
            0.0
        );

        // Distance between north & south poles.
        assert_relative_eq!(
            Norm::zero()
                .distance_to(Norm::zero().south(&half_turn))
                .to_f64()
                .unwrap(),
            2.0
        );

        // Distance between two opposite points on the equator.
        assert_relative_eq!(
            Norm::zero()
                .south(&quarter_turn)
                .distance_to(Norm::zero().south(&quarter_turn).west(&half_turn))
                .to_f64()
                .unwrap(),
            2.0
        );

        // Distance between any point on the equator and any pole.
        assert_relative_eq!(
            Norm::zero()
                .south(&half_turn)
                .distance_to(Norm::zero().south(&quarter_turn))
                .to_f64()
                .unwrap(),
            sqrt(2.into()).to_f64().unwrap()
        );
    }
}
