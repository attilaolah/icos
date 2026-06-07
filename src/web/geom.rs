use super::babylon::{xyz, xyzv};
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

type Vec3 = [Val; 3];

fn nvec(n: &Norm) -> Vec3 {
    [n.x(), n.y(), n.z()]
}

fn vadd(a: &Vec3, b: &Vec3) -> Vec3 {
    [
        a[0].add(&b[0]),
        a[1].add(&b[1]),
        a[2].add(&b[2]),
    ]
}

fn vsub(a: &Vec3, b: &Vec3) -> Vec3 {
    [
        a[0].sub(&b[0]),
        a[1].sub(&b[1]),
        a[2].sub(&b[2]),
    ]
}

fn vmul(a: &Vec3, s: &Val) -> Vec3 {
    [
        a[0].mul(s),
        a[1].mul(s),
        a[2].mul(s),
    ]
}

fn dot(a: &Vec3, b: &Vec3) -> Val {
    a[0].mul(&b[0]).add(&a[1].mul(&b[1])).add(&a[2].mul(&b[2]))
}

fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
    [
        a[1].mul(&b[2]).sub(&a[2].mul(&b[1])),
        a[2].mul(&b[0]).sub(&a[0].mul(&b[2])),
        a[0].mul(&b[1]).sub(&a[1].mul(&b[0])),
    ]
}

fn rotate_to_unit_sphere_around_line(point: &Vec3, line_a: &Vec3, line_b: &Vec3) -> Vec3 {
    let axis_raw = vsub(line_b, line_a);
    let axis_len = dot(&axis_raw, &axis_raw).sqrt();
    let axis = vmul(&axis_raw, &axis_len.rec());

    let v = vsub(point, line_a);
    let v_par = vmul(&axis, &dot(&v, &axis));
    let v_perp = vsub(&v, &v_par);
    let w = cross(&axis, &v);

    let k = vadd(line_a, &v_par);

    // Solve a*cos(t) + b*sin(t) = m for t,
    // where |line_a + R_axis(t) * (point - line_a)| = 1.
    // Since `point` itself is on the unit sphere, t=0 is always a solution.
    // We explicitly take the *other* branch (nontrivial intersection) to avoid
    // collapsing D->B and E->A.
    let a = dot(&k, &v_perp);
    let b = dot(&k, &w);
    let r2 = a.ipow(2).add(&b.ipow(2));
    let cos_t = a.ipow(2).sub(&b.ipow(2)).div(&r2);
    let sin_t = a.mul(&b).imul(2).div(&r2);

    let rotated = vadd(
        &v_par,
        &vadd(&vmul(&v_perp, &cos_t), &vmul(&w, &sin_t)),
    );
    vadd(line_a, &rotated)
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
        let by = alpha().mul(&t1).idiv(2);

        let fifth = Angle::turn().idiv(5);
        let top = Norm::zero();
        let pentagon = ((0 as i64)..5)
            .into_iter()
            .map(|i| top.south(&by).east(&fifth.imul(i)));

        let r_0_0 = top.south(&by);
        let r_0_1 = r_0_0.east(&fifth);
        // Enforce equilateral top: AB = AF (and by symmetry AB = BC as well).
        let cos_ab = by.cos().ipow(2).add(&by.sin().ipow(2).mul(&fifth.cos()));
        let delta = cos_ab.acos();
        let by_1 = by.add(&delta);
        let r_1_0 = top.south(&by_1);
        let r_1_1 = r_1_0.east(&fifth);
        let a = nvec(&r_0_0);
        let b = nvec(&r_0_1);
        let f = nvec(&r_1_0);
        let c = nvec(&r_1_1);

        // Bottom pair from rotating A/B around the CF axis until they hit the unit sphere.
        let e = rotate_to_unit_sphere_around_line(&a, &c, &f);
        let d = rotate_to_unit_sphere_around_line(&b, &c, &f);

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
                    positions: xyzv(vec![a.clone(), b.clone(), f.clone()]),
                    indices: vec![0, 2, 1],
                    symmetry: "icos.f.3".into(),
                },
                // Another outer-triangle seed in the same local face, rotated 3x via `icos.f.3`.
                Mesh {
                    positions: xyzv(vec![b.clone(), f.clone(), d.clone()]),
                    indices: vec![0, 1, 2],
                    symmetry: "icos.f.3".into(),
                },
                // Third outer-triangle seed, added on top of the existing scaffold.
                Mesh {
                    positions: xyzv(vec![b, d.clone(), c.clone()]),
                    indices: vec![0, 1, 2],
                    symmetry: "icos.f.3".into(),
                },
                // Fourth outer-triangle seed: F-D-E.
                Mesh {
                    positions: xyzv(vec![f, d.clone(), e.clone()]),
                    indices: vec![0, 2, 1],
                    symmetry: "icos.f.3".into(),
                },
                // one seed inner triangle, rotated from a single point via `icos.f.c`.
                Mesh {
                    positions: xyzv(vec![d]),
                    indices: vec![],
                    symmetry: "icos.f.c".into(),
                },
            ],
            // Temporary default for development; expected to be tuned.
            params: vec!["0.27".into()],
        }
    }
}
