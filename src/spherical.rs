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
        self.south(&a.clone().neg())
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
        self.east(&a.clone().neg())
    }

    pub fn rot_x(self, a: &Angle) -> Self {
        Self {
            theta: self
                .theta
                .clone()
                .sin()
                .mul(&self.phi.clone().sin())
                .mul(&a.clone().sin())
                .add(&self.theta.clone().cos().mul(&a.clone().cos()))
                .acos(),
            phi: self
                .phi
                .clone()
                .tan()
                .mul(
                    &a.clone().cos().sub(
                        &self
                            .phi
                            .clone()
                            .tan()
                            .rec()
                            .mul(&a.clone().sin())
                            .mul(&self.phi.clone().cos().rec()),
                    ),
                )
                .atan(),
        }
    }

    pub fn distance_to(self, to: Self) -> Val {
        let dx = to.x().sub(&self.x()).ipow(2);
        let dy = to.y().sub(&self.y()).ipow(2);
        let dz = to.z().sub(&self.z()).ipow(2);
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
    use approx::assert_relative_eq;
    use num_traits::ToPrimitive;

    #[test]
    fn test_distance_to() {
        let half_turn = Angle::turn().idiv(2);
        let quarter_turn = half_turn.clone().idiv(2);

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
            Val::from(2).sqrt().to_f64().unwrap()
        );
    }
}
