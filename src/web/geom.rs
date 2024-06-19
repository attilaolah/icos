use super::babylon::xyz;
use crate::{alpha, beta, Angle, Norm, Val};
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Geometry {
    positions: Vec<String>,
    indices: Vec<u64>,
    symmetry: String,
}

impl Geometry {
    pub fn goldberg_1_0() -> Vec<Self> {
        let fifth = Angle::turn().idiv(5);
        let tenth = Angle::turn().idiv(10);

        let top = Norm::zero();

        vec![Self {
            positions: xyz(((0 as i64)..5)
                .into_iter()
                .map(|i| {
                    top.clone()
                        .south(&beta())
                        .east(&tenth)
                        .east(&fifth.clone().imul(i))
                })
                .collect()),
            indices: vec![0, 1, 2, 2, 3, 0, 0, 3, 4],
            symmetry: "icos.v.1".into(),
        }]
    }

    pub fn goldberg_1_1() -> Vec<Self> {
        let t = Val::param(1);
        let by = alpha().mul(&t).idiv(2);

        let fifth = Angle::turn().idiv(5);

        let top = Norm::zero();
        let pentagon = ((0 as i64)..5)
            .into_iter()
            .map(|i| top.clone().south(&by).east(&fifth.clone().imul(i)));

        let r_0_0 = top.clone().south(&by);
        let r_0_1 = r_0_0.clone().east(&fifth);
        let r_1_0 = top.clone().south(&alpha()).north(&by);

        vec![
            Self {
                positions: xyz(pentagon.collect()),
                indices: vec![0, 1, 2, 2, 3, 0, 0, 3, 4],
                symmetry: "icos.v.1".into(),
            },
            Self {
                positions: xyz(vec![r_0_0, r_0_1.clone(), r_1_0]),
                indices: vec![0, 2, 1],
                symmetry: "icos.f.3".into(),
            },
            Self {
                positions: xyz(vec![r_0_1]),
                indices: vec![],
                symmetry: "icos.f.c".into(),
            },
        ]
    }

    pub fn goldberg_2_0() -> Vec<Self> {
        let t = Val::param(1);
        let by = beta().mul(&t);

        let fifth = Angle::turn().idiv(5);
        let tenth = Angle::turn().idiv(10);

        let o = Norm::zero().south(&beta()).east(&tenth);
        let r_0 = Norm::zero().south(&by).east(&tenth);

        vec![
            Self {
                positions: xyz(((0 as i64)..5)
                    .into_iter()
                    .map(|i| r_0.clone().east(&fifth.clone().imul(i)))
                    .collect()),
                indices: vec![0, 1, 2, 2, 3, 0, 0, 3, 4],
                symmetry: "icos.v.1".into(),
            },
            Self {
                positions: xyz(vec![
                    r_0.clone(),
                    r_0.clone().east(&fifth),
                    o.clone(),
                    o.clone().east(&fifth),
                ]),
                indices: vec![1, 0, 2, 1, 2, 3],
                symmetry: "icos.f.3".into(),
            },
        ]
    }
}
