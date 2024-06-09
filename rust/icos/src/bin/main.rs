use icos::{alpha, Angle, Norm, Val};
use num_traits::ToPrimitive;
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
        .mount("/geometry", routes![icos_json])
}

#[get("/icos.f64.json")]
fn icos_json() -> Json<F64Geometry> {
    Json(F64Geometry::icos())
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct F64Geometry {
    positions: Vec<f64>,
    indices: Vec<u64>,
}

impl F64Geometry {
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

        Self {
            positions: vec![
                &top, //
                &top_0, &top_1, &top_2, &top_3, &top_4, //
                &bot_0, &bot_1, &bot_2, &bot_3, &bot_4, //
                &bot,
            ]
            .into_iter()
            .map(|n| [n.x(), n.y(), n.z()])
            .flatten()
            .map(|v| v.to_f64().unwrap_or(0.0))
            .collect(),
            indices: vec![
                0, 2, 1, 0, 3, 2, 0, 4, 3, 0, 5, 4, 0, 1, 5, // 1st round
                1, 2, 6, 2, 3, 7, 3, 4, 8, 4, 5, 9, 5, 1, 10, // 2nd round
                1, 6, 10, 2, 7, 6, 3, 8, 7, 4, 9, 8, 5, 10, 9, // 3rd round
                6, 7, 11, 7, 8, 11, 8, 9, 11, 9, 10, 11, 10, 6, 11, // 4th round
            ],
        }
    }
}

fn _calculate() {
    let mut step = 1;
    let max_steps = 60;

    let fifth_turn = Angle::turn().div(&5.into());

    let mut t: Val = 1.into();
    let mut adjust = t.clone();

    loop {
        let by = alpha().mul(&t).div(&2.into());

        let a = Norm::zero().south(&by);
        let b = Norm::zero().south(&alpha()).north(&by);
        let c = a.clone().east(&fifth_turn);

        let ab = a.clone().distance_to(b);
        let ac = a.clone().distance_to(c);
        let delta = ab.clone().sub(&ac);

        println!(
            "{:0.16}: {:0.16} - {:0.16} = {:+0.16}",
            t.to_f64().unwrap(),
            ab.clone().to_f64().unwrap(),
            ac.clone().to_f64().unwrap(),
            delta.to_f64().unwrap(),
        );

        adjust = adjust.div(&2.into());
        t = if delta.to_f64().unwrap() > 0.0 {
            t.add(&adjust)
        } else {
            t.sub(&adjust)
        };

        step += 1;
        if step > max_steps {
            break;
        }
    }
}

fn _print_formula() {
    let fifth_turn = Angle::turn().div(&5.into());

    let t = Val::param(1);
    let by = alpha().mul(&t).div(&2.into());

    let a = Norm::zero().south(&by);
    let b = Norm::zero().south(&alpha()).north(&by);
    let c = a.clone().east(&fifth_turn);

    let ab = a.clone().distance_to(b);
    let ac = a.clone().distance_to(c);
    let delta = ab.clone().sub(&ac);

    println!("function f({}) {} endfunction", t, delta);
}
