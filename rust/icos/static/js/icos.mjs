const canvas = document.getElementById("viewport");
let engine, scene, resize;

async function draw(shape) {
  if (engine) {
    scene.dispose();
    engine.dispose();

    window.removeEventListener("resize", resize);
  }

  engine = new BABYLON.Engine(canvas, true);

  scene = new BABYLON.Scene(engine);
  scene.clearColor = BABYLON.Color3.Black();

  const light = new BABYLON.HemisphericLight(
    "HemiLight",
    new BABYLON.Vector3(0, 4, 2),
    scene,
  );
  light.intensity = 0.8;

  const req = await fetch(`/geometry/${shape}.json`);
  const geometry = await req.json();

  const camera = new BABYLON.ArcRotateCamera("Camera",
    0, 0, 4,
    new BABYLON.Vector3(0, 0, 0),
    scene,
  );
  camera.setPosition(new BABYLON.Vector3(0, 2, -4));
  camera.lowerRadiusLimit = 2;
  camera.upperRadiusLimit = 20;
  camera.attachControl(canvas, true);

  const data = new BABYLON.VertexData();
  data.positions = geometry.positions;
  data.indices = geometry.indices;

  const mesh = new BABYLON.Mesh("Icosahedron", scene);
  data.applyToMesh(mesh);

  // Register a render loop to repeatedly render the scene
  engine.runRenderLoop(function() {
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
