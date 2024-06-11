const canvas = document.getElementById("viewport");
let engine, scene, resize;

async function draw(shape) {
  const { ArcRotateCamera, Color3, Engine, HemisphericLight, Mesh, Scene, Vector3, VertexData } = BABYLON;

  if (engine) {
    scene.dispose();
    engine.dispose();

    window.removeEventListener("resize", resize);
  }

  const req = fetch(`/geometry/${shape}.json`);

  engine = new Engine(canvas, true);
  scene = new Scene(engine);
  scene.clearColor = Color3.Black();

  const light = new HemisphericLight(
    "HemiLight",
    new Vector3(0, 4, 2),
    scene,
  );
  light.intensity = 0.8;

  document.body.querySelectorAll(".params label")
    .forEach(elem => {
      elem.style.display = "none";
    })
  document.body.querySelectorAll(`.params label.${shape.replace(".", "-")}`)
    .forEach(elem => {
      elem.style.display = "initial";
    })

  const camera = new ArcRotateCamera("Camera",
    0, 2, 0,
    new Vector3(0, 0, 0),
    scene,
  );
  camera.setPosition(new Vector3(0, 2, -4));
  camera.lowerRadiusLimit = 2;
  camera.upperRadiusLimit = 20;
  camera.attachControl(canvas, true);

  const meshes = Array(20).fill().map(() => new Mesh("Icosahedron", scene));

  const geometry = await (await req).json();
  const positions = geometry.positions
    .map(pos => new Function("t_1", `with (Math) return (${pos});`));
  symmetry(meshes, geometry.symmetry);

  const data = new VertexData();
  data.indices = geometry.indices;

  let t;
  const update = () => {
    const newt = parseFloat(document.getElementById("t").value);
    if (newt === t) return;
    t = newt;

    data.positions = positions.map(fn => fn(t));
    meshes.forEach(m => data.applyToMesh(m));
  };

  update();

  // Register a render loop to repeatedly render the scene
  engine.runRenderLoop(() => {
    update();
    scene.render();
  });

  // Watch for browser/canvas resize events
  resize = window.addEventListener("resize", function() {
    engine.resize();
  });
}

const shape = document.getElementById("shape");
shape.addEventListener("change", () => draw(shape.value));
await draw(shape.value);

function symmetry(meshes, sym) {
  const { Space, Vector3 } = BABYLON;
  const { PI, acos, cos, pow, sin } = Math;
  const { WORLD } = Space;

  if (sym !== "icos.20") {
    throw new Error(`symmetry not supported: ${sym}`);
  }

  const a = new Vector3(
    sin(acos((pow(5, (1 / 2)) * (1 / 5)))) * cos((2 * (1 / 5)) * PI),
    cos(acos((pow(5, (1 / 2)) * (1 / 5)))),
    sin(acos((pow(5, (1 / 2)) * (1 / 5)))) * sin((2 * (1 / 5)) * PI),
  );
  const x = new Vector3(1, 0, 0);
  const y = new Vector3(0, 1, 0);

  for (let i = 0; i < 5; i++) {
    meshes[i]
      .rotate(y, PI / 5 * 2 * i, WORLD);
    meshes[i + 5]
      .rotate(a, -PI / 5 * 2, WORLD)
      .rotate(y, PI / 5 * 2 * i, WORLD);
    meshes[i + 10]
      .rotate(a, -PI / 5 * 2, WORLD)
      .rotate(x, PI, WORLD)
      .rotate(y, PI / 5 * (i * 2 + 1), WORLD);
    meshes[i + 15]
      .rotate(x, PI, WORLD)
      .rotate(y, PI / 5 * (i * 2 + 1), WORLD);
  }
}
