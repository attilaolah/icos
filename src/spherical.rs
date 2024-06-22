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

    pub fn north(&self, a: &Angle) -> Self {
        self.south(&a.clone().neg())
    }

    pub fn south(&self, a: &Angle) -> Self {
        Self {
            theta: self.theta.add(a),
            phi: self.phi.clone(),
        }
    }

    pub fn east(&self, a: &Angle) -> Self {
        Self {
            theta: self.theta.clone(),
            phi: self.phi.add(a),
        }
    }

    pub fn west(&self, a: &Angle) -> Self {
        self.east(&a.clone().neg())
    }

    pub fn rot_x(&self, by: &Angle) -> Self {
        Self {
            theta: self
                .theta
                .sin()
                .mul(&self.phi.sin())
                .mul(&by.sin())
                .add(&self.theta.cos().mul(&by.cos()))
                .acos(),
            phi: self
                .phi
                .tan()
                .mul(
                    &by.cos().sub(
                        &self
                            .phi
                            .tan()
                            .rec()
                            .mul(&by.sin())
                            .mul(&self.phi.cos().rec()),
                    ),
                )
                .atan(),
        }
    }

    /// Rotate using quaternion multiplications.
    pub fn rot(&self, axis: &Self, by: &Angle) -> Self {
        if by.is_zero() {
            return self.clone();
        }

        // The point to rotate.
        // Note the negative z value, to preserve z-up axis.
        let p = [Val::from(0), self.x(), self.y(), self.z().neg()];

        let sin = by.idiv(2).sin();
        // Te quaternion to rotate by:
        let q = [
            by.idiv(2).cos(),
            axis.x().mul(&sin),
            axis.y().mul(&sin),
            axis.z().mul(&sin),
        ];
        // The inverse of the rotation quaternion:
        let qn = [q[0].clone(), q[1].neg(), q[2].neg(), q[3].neg()];

        // Result of multiplying q with p (considering p[0] == 0):
        let qp = [
            (q[1].mul(&p[1]).neg())
                .sub(&q[2].mul(&p[2]))
                .sub(&q[3].mul(&p[3])),
            q[0].mul(&p[1]).sub(&q[2].mul(&p[3])).add(&q[3].mul(&p[2])),
            q[0].mul(&p[2]).add(&q[1].mul(&p[3])).sub(&q[3].mul(&p[1])),
            q[0].mul(&p[3]).sub(&q[1].mul(&p[2])).add(&q[2].mul(&p[1])),
        ];

        // Result of multiplying qp with pn:
        let qpqn = [
            Val::from(0),
            // I.e. the result of:
            // (qp[0].mul(&qn[0]))
            //     .sub(&qp[1].mul(&qn[1]))
            //     .sub(&qp[2].mul(&qn[2]))
            //     .sub(&qp[3].mul(&qn[3])),
            (qp[0].mul(&qn[1]))
                .add(&qp[1].mul(&qn[0]))
                .sub(&qp[2].mul(&qn[3]))
                .add(&qp[3].mul(&qn[2])),
            (qp[0].mul(&qn[2]))
                .add(&qp[1].mul(&qn[3]))
                .add(&qp[2].mul(&qn[0]))
                .sub(&qp[3].mul(&qn[1])),
            (qp[0].mul(&qn[3]))
                .add(&qp[1].mul(&qn[2]))
                .add(&qp[2].mul(&qn[1]))
                .sub(&qp[3].mul(&qn[0])),
        ];

        // Convert back to axis-angle:
        Self {
            theta: qpqn[3]
                .div(
                    &(qpqn[1].ipow(2))
                        .add(&qpqn[2].ipow(2))
                        .add(&qpqn[3].ipow(2))
                        .sqrt(),
                )
                .acos(),
            phi: qpqn[1]
                .div(&(qpqn[1].ipow(2)).add(&qpqn[2].ipow(2)).sqrt())
                .acos(),
        }
    }

    pub fn distance_to(self, to: Self) -> Val {
        let dx = to.x().sub(&self.x()).ipow(2);
        let dy = to.y().sub(&self.y()).ipow(2);
        let dz = to.z().sub(&self.z()).ipow(2);
        dx.add(&dy).add(&dz).sqrt()
    }

    pub fn x(&self) -> Val {
        self.theta.sin().mul(&self.phi.cos())
    }

    pub fn y(&self) -> Val {
        self.theta.sin().mul(&self.phi.sin())
    }

    pub fn z(&self) -> Val {
        self.theta.cos()
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
