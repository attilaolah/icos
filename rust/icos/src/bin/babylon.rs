use icos::{alpha, beta, Angle, Norm, Val};
use rocket::{
    fs::{relative, FileServer},
    serde::{json::Json, Serialize},
};

#[macro_use]
extern crate rocket;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Geometry {
    positions: Vec<String>,
    indices: Vec<u64>,
    symmetry: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Consts {
    x: Vec<String>,
    y: Vec<String>,
    z: Vec<String>,
    r: Vec<String>,
    o: Vec<String>,
}

#[get("/icos.json")]
fn icos_json() -> Json<Vec<Geometry>> {
    let a = alpha();
    let fifth = Angle::turn().div(&5.into());

    let top = Norm::zero();

    let positions = vec![
        top.clone(),
        top.clone().south(&a),
        top.south(&a).east(&fifth),
    ];

    Json(vec![Geometry {
        positions: xyz(positions),
        indices: vec![0, 1, 2],
        symmetry: "icos.f.1".into(),
    }])
}

#[get("/icos.trunc.json")]
fn icos_trunc_json() -> Json<Vec<Geometry>> {
    let t = Val::param(1);
    let by = alpha().mul(&t).div(&2.into());

    let fifth = Angle::turn().div(&5.into());

    let top = Norm::zero();
    let pentagon = ((0 as i64)..5)
        .into_iter()
        .map(|i| top.clone().south(&by).east(&fifth.clone().mul(&i.into())));

    let r_0_0 = top.clone().south(&by);
    let r_0_1 = r_0_0.clone().east(&fifth);
    let r_1_0 = top.clone().south(&alpha()).north(&by);

    Json(vec![
        Geometry {
            positions: xyz(pentagon.collect()),
            indices: vec![0, 1, 2, 2, 3, 0, 0, 3, 4],
            symmetry: "icos.v.1".into(),
        },
        Geometry {
            positions: xyz(vec![r_0_0, r_0_1.clone(), r_1_0]),
            indices: vec![0, 2, 1],
            symmetry: "icos.f.3".into(),
        },
        Geometry {
            positions: xyz(vec![r_0_1]),
            indices: vec![],
            symmetry: "icos.f.c".into(),
        },
    ])
}

#[get("/dodec.json")]
fn dodec_json() -> Json<Vec<Geometry>> {
    let fifth = Angle::turn().div(&5.into());
    let tenth = Angle::turn().div(&10.into());

    let top = Norm::zero();
    let pentagon = ((0 as i64)..5).into_iter().map(|i| {
        top.clone()
            .south(&beta())
            .east(&tenth)
            .east(&fifth.clone().mul(&i.into()))
    });

    Json(vec![Geometry {
        positions: xyz(pentagon.collect()),
        indices: vec![0, 1, 2, 2, 3, 0, 0, 3, 4],
        symmetry: "icos.v.1".into(),
    }])
}

#[get("/consts.json")]
fn consts_json() -> Json<Consts> {
    let z = Norm::zero();

    let q = z.clone().south(&alpha());
    let r = q.clone().east(&Angle::turn().div(&5.into()));

    let ox = format!("{} + {}", q.x(), r.x());
    let oy = r.y().to_string();
    let oz = format!("{} + {} + {}", q.z(), r.z(), z.z());

    Json(Consts {
        x: vec!["1".into(), "0".into(), "0".into()],
        y: vec!["0".into(), "1".into(), "0".into()],
        z: vec!["0".into(), "0".into(), "1".into()],
        r: xyz(vec![r]),
        o: vec![ox, oz, oy],
    })
}

fn xyz(points: Vec<Norm>) -> Vec<String> {
    points
        .into_iter()
        // BABYLON is Y-up left-handed.
        .map(|n| [n.x(), n.z(), n.y()])
        .flatten()
        .map(|v| format!("{}", v))
        .collect()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount(
            "/geometry",
            routes![icos_json, icos_trunc_json, dodec_json, consts_json],
        )
}
