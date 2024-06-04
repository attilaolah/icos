use icos::{alpha, Angle, Norm, Val};
use num_traits::ToPrimitive;
use rocket::{
    response::content::RawHtml,
    serde::{json::Json, Serialize},
};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(
        r#"
<style>
  html,
  body {
    overflow: hidden;
    width: 100%;
    height: 100%;
    margin: 0;
    padding: 0;
  }

  #viewport {
    width: 100%;
    height: 100%;
    touch-action: none;
  }
</style>

<script src="https://cdn.babylonjs.com/babylon.js"></script>
<script src="https://cdn.babylonjs.com/loaders/babylonjs.loaders.min.js"></script>
<script src="https://code.jquery.com/pep/0.4.3/pep.js"></script>

<canvas id="viewport" touch-action="none"></canvas>

<script type="module">
  const canvas = document.getElementById("viewport");
  const engine = new BABYLON.Engine(canvas, true);
  const scene = new BABYLON.Scene(engine);

  const light = new BABYLON.HemisphericLight('HemiLight',
      new BABYLON.Vector3(0, 4, 2),
      scene);
  light.intensity = 0.8;

  const req = await fetch("/geometry/icos.f64.json");
  const geometry = await req.json();

  const camera = new BABYLON.ArcRotateCamera('Camera', 0, 0, 0, new BABYLON.Vector3(0, 0, 0), scene);
  camera.setPosition(new BABYLON.Vector3(0, 5, -30));
  camera.attachControl(canvas, true);

  const data = new BABYLON.VertexData();
  data.positions = geometry.positions;
  data.indices = geometry.indices;

  const mesh = new BABYLON.Mesh('Icosahedron', scene);
  data.applyToMesh(mesh);

  // Register a render loop to repeatedly render the scene
  engine.runRenderLoop(function () {
    scene.render();
  });

  // Watch for browser/canvas resize events
  window.addEventListener("resize", function () {
    engine.resize();
  });
</script>
"#,
    )
}

#[get("/geometry/icos.f64.json")]
fn icos_json() -> Json<Geometry> {
    Json(Geometry::icos())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, icos_json])
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Geometry {
    positions: Vec<f64>,
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
                0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 1, // 1st round
                1, 6, 2, 2, 7, 3, 3, 8, 4, 4, 9, 5, 5, 10, 1, // 2nd round
                1, 10, 6, 2, 6, 7, 3, 7, 8, 4, 8, 9, 5, 9, 10, // 3rd round
                6, 11, 7, 7, 11, 8, 8, 11, 9, 9, 11, 10, 10, 11, 6, // 4th round
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
