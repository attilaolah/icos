use super::babylon::xyz;
use crate::{alpha, beta, Angle, Norm, Val};
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Mesh {
    positions: Vec<String>,
    indices: Vec<u64>,
    symmetry: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Geometry {
    meshes: Vec<Mesh>,
    params: Vec<String>,
}

impl Geometry {
    pub fn goldberg_1_0() -> Self {
        let fifth = Angle::part(5);
        let tenth = Angle::part(10);

        let top = Norm::zero();

        Self {
            meshes: vec![Mesh {
                positions: xyz(((0 as i64)..5)
                    .into_iter()
                    .map(|i| top.south(&beta()).east(&tenth).east(&fifth.imul(i)))
                    .collect()),
                indices: vec![0, 1, 2, 2, 3, 0, 0, 3, 4],
                symmetry: "icos.v.1".into(),
            }],
            params: vec![],
        }
    }

    pub fn goldberg_1_1() -> Self {
        let t = Val::param(1);
        let by = alpha().mul(&t).idiv(2);

        let fifth = Angle::turn().idiv(5);

        let top = Norm::zero();
        let pentagon = ((0 as i64)..5)
            .into_iter()
            .map(|i| top.south(&by).east(&fifth.imul(i)));

        let r_0_0 = top.south(&by);
        let r_0_1 = r_0_0.east(&fifth);
        let r_1_0 = top.south(&alpha()).north(&by);

        Self {
            meshes: vec![
                Mesh {
                    positions: xyz(pentagon.collect()),
                    indices: vec![0, 1, 2, 2, 3, 0, 0, 3, 4],
                    symmetry: "icos.v.1".into(),
                },
                Mesh {
                    positions: xyz(vec![r_0_0, r_0_1.clone(), r_1_0]),
                    indices: vec![0, 2, 1],
                    symmetry: "icos.f.3".into(),
                },
                Mesh {
                    positions: xyz(vec![r_0_1]),
                    indices: vec![],
                    symmetry: "icos.f.c".into(),
                },
            ],
            params: vec!["0.6329870724964068".into()],
        }
    }

    pub fn goldberg_2_0() -> Self {
        let t = Val::param(1);
        let by = beta().mul(&t);

        let fifth = Angle::part(5);
        let tenth = Angle::part(10);

        let o = Norm::zero().south(&beta()).east(&tenth);
        let r_0 = Norm::zero().south(&by).east(&tenth);

        Self {
            meshes: vec![
                Mesh {
                    positions: xyz(((0 as i64)..5)
                        .into_iter()
                        .map(|i| r_0.east(&fifth.imul(i)))
                        .collect()),
                    indices: vec![0, 1, 2, 2, 3, 0, 0, 3, 4],
                    symmetry: "icos.v.1".into(),
                },
                Mesh {
                    positions: xyz(vec![
                        r_0.clone(),
                        r_0.east(&fifth),
                        o.clone(),
                        o.east(&fifth),
                    ]),
                    indices: vec![1, 0, 2, 1, 2, 3],
                    symmetry: "icos.f.3".into(),
                },
            ],
            params: vec!["0.42".into()],
        }
    }

    pub fn goldberg_2_2() -> Self {
        let t1 = Val::param(1);
        let t2 = Val::param(2);
        let t3 = Val::param(3);
        let t4 = Val::param(4);

        let by = alpha().mul(&t1).idiv(2);
        let mid = alpha().idiv(2);
        // Second pair slides from the pentagon edge (by) to the icosahedral midpoint (alpha/2).
        let by_1 = by.add(&mid.sub(&by).mul(&t2));

        let fifth = Angle::turn().idiv(5);
        let top = Norm::zero();
        let pentagon = ((0 as i64)..5)
            .into_iter()
            .map(|i| top.south(&by).east(&fifth.imul(i)));

        let r_0_0 = top.south(&by);
        let r_0_1 = r_0_0.east(&fifth);
        let r_1_0 = top.south(&by_1);
        let r_1_1 = r_1_0.east(&fifth);

        // Third pair latitude is constrained:
        // t3=0  => same polar distance as C/F (by_1)
        // t3=1  => icosahedral face-center latitude (beta)
        let theta_2 = by_1.add(&beta().sub(&by_1).mul(&t3));
        let phi_2_0 = t4.pi();
        let phi_2_1 = fifth.sub(&phi_2_0);
        let r_2_0 = Norm::zero().south(&theta_2).east(&phi_2_0);
        let r_2_1 = Norm::zero().south(&theta_2).east(&phi_2_1);

        Self {
            meshes: vec![
                Mesh {
                    positions: xyz(pentagon.collect()),
                    indices: vec![0, 1, 2, 2, 3, 0, 0, 3, 4],
                    symmetry: "icos.v.1".into(),
                },
                // First hexagonal face scaffold:
                // one seed outer triangle, rotated 3x via `icos.f.3`.
                Mesh {
                    positions: xyz(vec![r_0_0.clone(), r_0_1.clone(), r_1_0.clone()]),
                    indices: vec![0, 2, 1],
                    symmetry: "icos.f.3".into(),
                },
                // Another outer-triangle seed in the same local face, rotated 3x via `icos.f.3`.
                Mesh {
                    positions: xyz(vec![r_0_1.clone(), r_1_0.clone(), r_2_0.clone()]),
                    indices: vec![0, 1, 2],
                    symmetry: "icos.f.3".into(),
                },
                // Third outer-triangle seed, added on top of the existing scaffold.
                Mesh {
                    positions: xyz(vec![r_0_1, r_2_0.clone(), r_1_1]),
                    indices: vec![0, 1, 2],
                    symmetry: "icos.f.3".into(),
                },
                // Fourth outer-triangle seed: F-D-E.
                Mesh {
                    positions: xyz(vec![r_1_0, r_2_0.clone(), r_2_1.clone()]),
                    indices: vec![0, 2, 1],
                    symmetry: "icos.f.3".into(),
                },
                // one seed inner triangle, rotated from a single point via `icos.f.c`.
                Mesh {
                    positions: xyz(vec![r_2_0]),
                    indices: vec![],
                    symmetry: "icos.f.c".into(),
                },
            ],
            // Temporary default for development; expected to be tuned.
            params: vec!["0.27".into(), "0.45".into(), "0.14".into(), "0.26".into()],
        }
    }
}
