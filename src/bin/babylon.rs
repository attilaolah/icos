use icos::web::{Consts, Geometry};
use rocket::{
    fs::{relative, FileServer},
    serde::json::Json,
};

#[macro_use]
extern crate rocket;

#[get("/consts.json")]
fn consts_json() -> Json<Consts> {
    Json(Consts::new())
}

#[get("/goldberg.1.0.json")]
fn goldberg_1_0_json() -> Json<Vec<Geometry>> {
    Json(Geometry::goldberg_1_0())
}

#[get("/goldberg.1.1.json")]
fn goldberg_1_1_json() -> Json<Vec<Geometry>> {
    Json(Geometry::goldberg_1_1())
}

#[get("/goldberg.2.0.json")]
fn goldberg_2_0_json() -> Json<Vec<Geometry>> {
    Json(Geometry::goldberg_2_0())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount(
            "/geometry",
            routes![
                consts_json,
                goldberg_1_0_json,
                goldberg_1_1_json,
                goldberg_2_0_json,
            ],
        )
}
