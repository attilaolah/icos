use crate::{alpha, Angle, Norm};
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Consts {
    x: Vec<String>,
    y: Vec<String>,
    z: Vec<String>,
    r: Vec<String>,
    o: Vec<String>,
}

impl Consts {
    pub fn new() -> Self {
        let z = Norm::zero();

        let q = z.clone().south(&alpha());
        let r = q.clone().east(&Angle::turn().div(&5.into()));

        let ox = format!("{} + {}", q.x(), r.x());
        let oy = r.y().to_string();
        let oz = format!("{} + {} + {}", q.z(), r.z(), z.z());

        Self {
            x: vec!["1".into(), "0".into(), "0".into()],
            y: vec!["0".into(), "1".into(), "0".into()],
            z: vec!["0".into(), "0".into(), "1".into()],
            r: xyz(vec![r]),
            o: vec![ox, oz, oy],
        }
    }
}

pub fn xyz(points: Vec<Norm>) -> Vec<String> {
    points
        .into_iter()
        // BABYLON is Y-up left-handed.
        .map(|n| [n.x(), n.z(), n.y()])
        .flatten()
        .map(|v| format!("{}", v))
        .collect()
}
