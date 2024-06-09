use icos::{alpha, Angle, Norm, Val};
use rocket::{
    fs::{relative, FileServer},
    serde::{json::Json, Serialize},
};

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/geometry", routes![icos_json, icos_trunc_json])
}

#[get("/icos.json")]
fn icos_json() -> Json<Geometry> {
    Json(Geometry::icos())
}

#[get("/icos.trunc.json")]
fn icos_trunc_json() -> Json<Geometry> {
    Json(Geometry::icos_trunc())
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Geometry {
    positions: Vec<String>,
    indices: Vec<u64>,
}

impl Geometry {
    fn icos() -> Self {
        let fifth_turn = Angle::turn().div(&5.into());

        let top = Norm::zero();
        let top_0 = top.clone().south(&alpha());
        let top_1 = top_0.clone().east(&fifth_turn);
        let top_2 = top_1.clone().east(&fifth_turn);
        let top_3 = top_2.clone().east(&fifth_turn);
        let top_4 = top_3.clone().east(&fifth_turn);

        let bot = Norm::zero().south(&Angle::turn().div(&2.into()));
        let bot_0 = bot
            .clone()
            .north(&alpha())
            .east(&fifth_turn.clone().div(&2.into()));
        let bot_1 = bot_0.clone().east(&fifth_turn);
        let bot_2 = bot_1.clone().east(&fifth_turn);
        let bot_3 = bot_2.clone().east(&fifth_turn);
        let bot_4 = bot_3.clone().east(&fifth_turn);

        let positions = vec![
            &top, //
            &top_0, &top_1, &top_2, &top_3, &top_4, //
            &bot_0, &bot_1, &bot_2, &bot_3, &bot_4, //
            &bot,
        ];

        Self {
            positions: positions
                .into_iter()
                .map(|n| [n.x(), n.y(), n.z()])
                .flatten()
                .map(|v| format!("{}", v))
                .collect(),
            indices: vec![
                0, 2, 1, 0, 3, 2, 0, 4, 3, 0, 5, 4, 0, 1, 5, // 1st round
                1, 2, 6, 2, 3, 7, 3, 4, 8, 4, 5, 9, 5, 1, 10, // 2nd round
                1, 6, 10, 2, 7, 6, 3, 8, 7, 4, 9, 8, 5, 10, 9, // 3rd round
                6, 7, 11, 7, 8, 11, 8, 9, 11, 9, 10, 11, 10, 6, 11, // 4th round
            ],
        }
    }

    fn icos_trunc() -> Self {
        let t = Val::param(1);
        let by = alpha().mul(&t).div(&2.into());

        let fifth_turn = Angle::turn().div(&5.into());

        let top = Norm::zero();

        let r_0_0 = top.clone().south(&by);
        let r_0_1 = r_0_0.clone().east(&fifth_turn);
        let r_0_2 = r_0_1.clone().east(&fifth_turn);
        let r_0_3 = r_0_2.clone().east(&fifth_turn);
        let r_0_4 = r_0_3.clone().east(&fifth_turn);

        let r_1_0 = top.clone().south(&alpha()).north(&by);
        let r_1_1 = r_1_0.clone().east(&fifth_turn);
        let r_1_2 = r_1_1.clone().east(&fifth_turn);
        let r_1_3 = r_1_2.clone().east(&fifth_turn);
        let r_1_4 = r_1_3.clone().east(&fifth_turn);

        let r_2_0 = top.clone().south(&alpha()).east(&by);
        let r_2_1 = top.clone().south(&alpha()).east(&alpha()).west(&by);
        let r_2_2 = r_2_0.clone().east(&fifth_turn);
        let r_2_3 = r_2_1.clone().east(&fifth_turn);
        let r_2_4 = r_2_2.clone().east(&fifth_turn);
        let r_2_5 = r_2_3.clone().east(&fifth_turn);
        let r_2_6 = r_2_4.clone().east(&fifth_turn);
        let r_2_7 = r_2_5.clone().east(&fifth_turn);
        let r_2_8 = r_2_6.clone().east(&fifth_turn);
        let r_2_9 = r_2_7.clone().east(&fifth_turn);

        let positions = vec![
            &r_0_0, &r_0_1, &r_0_2, &r_0_3, &r_0_4, // 1st row
            &r_1_0, &r_1_1, &r_1_2, &r_1_3, &r_1_4, // 2nd row
            &r_2_0, &r_2_1, &r_2_2, &r_2_3, &r_2_4, &r_2_5, &r_2_6, &r_2_7, &r_2_8, &r_2_9, //
        ];

        Self {
            positions: positions
                .into_iter()
                .map(|n| [n.x(), n.y(), n.z()])
                .flatten()
                .map(|v| format!("{}", v))
                .collect(),
            indices: vec![
                0, 2, 1, 4, 2, 0, 3, 2, 4, // top pentagon
                // hexagons top part:
                0, 1, 5, 1, 6, 5, //
                1, 2, 6, 2, 7, 6, //
                2, 3, 7, 3, 8, 7, //
                3, 4, 8, 4, 9, 8, //
                4, 0, 9, 0, 5, 9, //
                // hexagons bottom part:
                5, 6, 10, 6, 11, 10, //
                6, 7, 12, 7, 13, 12, //
                7, 8, 14, 8, 15, 14, //
                8, 9, 16, 9, 17, 16, //
                9, 5, 18, 5, 19, 18, //
                // top parts of top-ring pentagons:
                5, 10, 19, 6, 12, 11, 7, 14, 13, 8, 16, 15, 9, 18, 17, //
            ],
        }
    }
}
