const { PI } = Math;
const { ArcRotateCamera, Color4, Engine, HemisphericLight, Mesh, MeshBuilder, Quaternion, Scene, Space, Vector3, VertexData } = BABYLON;
const { WORLD } = Space;
const { X, Y, R, O } = await consts();

const canvas = document.getElementById("viewport");
let engine, scene, resize;

async function consts() {
  return Object.fromEntries(Object
    .entries(await (await fetch("/geometry/consts.json")).json())
    .map(([key, val]) => [
      key.toUpperCase(),
      pvec(val),
    ]));
}

function pfun(pos, params) {
  return new Function(...
    Array(params)
      .fill()
      .map((_, i) => `t_${i + 1}`)
      .concat(`with (Math) return (${pos});`));
}

function pval(pos) {
  return pfun(pos, 0)();
}

function pvec(xyz) {
  return new Vector3(...xyz.map(pval));
}

async function draw(shape) {
  if (engine) {
    scene.dispose();
    engine.dispose();

    window.removeEventListener("resize", resize);
  }

  const req = fetch(`/geometry/${shape}.json`);

  engine = new Engine(canvas, true);
  scene = new Scene(engine);
  scene.clearColor = new Color4(0, 0, 0, 0);

  const ul = new HemisphericLight(
    "UpLight",
    new Vector3(0, 4, 2),
    scene,
  );
  ul.intensity = 0.8;

  const fl = new HemisphericLight(
    "FrontLight",
    new Vector3(-2, 0, -2),
    scene,
  );
  fl.intensity = 0.2;

  const camera = new ArcRotateCamera("Camera",
    0, 2, 0,
    new Vector3(0, 0, 0),
    scene,
  );
  camera.setPosition(new Vector3(0, 2, -4));
  camera.lowerRadiusLimit = 2;
  camera.upperRadiusLimit = 20;
  camera.attachControl(canvas, true);

  const geometry = await (await req).json();
  const updates = geometry.meshes.map(data => {
    let params = geometry.params.map(pval);
    data.positions = data.positions.map(pos => pfun(pos, params.length));
    const meshes = symmetry(data);

    const vd = new VertexData();
    vd.indices = data.indices;

    let update = true;
    return () => {
      params.forEach((t, i) => {
        const newt = parseFloat(document.getElementById(`t${i}`).value);
        if (newt !== t) {
          document.querySelector(`output[for=t${i}]`).value = newt.toFixed(2);
          params[i] = newt;
          update = true;
        }
      });

      if (!update) return;
      update = false;

      vd.positions = data.positions.map(fn => fn.apply(null, params));
      meshes.forEach(m => {
        if (m.name.startsWith("dbg.")) {
          m.position = pvec(vd.positions);
        } else {
          vd.applyToMesh(m);
        }
      });
    };
  });

  const params = document.body.querySelector(".params");
  Array.from(params.children).forEach(elem => elem.remove());
  params.append(...geometry.params.map((t, i) => {
    const sub = document.createElement("sub");
    sub.innerText = i + 1;

    const em = document.createElement("em");
    em.append("t", sub);

    const name = `t${i}`;
    const output = document.createElement("output");
    output.setAttribute("for", name);
    output.value = pval(t).toFixed(2);

    const input = document.createElement("input");
    Object.entries({
      name,
      id: name,
      type: "range",
      step: "0.01",
      min: "0",
      max: "1",
      value: t,
    }).forEach(([key, val]) => input.setAttribute(key, val));


    const label = document.createElement("label");
    label.append(em, " = ", output, input);

    return label;
  }));

  engine.runRenderLoop(() => {
    updates.forEach(fn => fn());
    scene.render();
  });

  resize = window.addEventListener("resize", function() {
    engine.resize();
  });
}

const shape = document.getElementById("shape");
shape.addEventListener("change", () => draw(shape.value));
await draw(shape.value);

function symmetry(mesh) {
  switch (mesh.symmetry) {
    case "icos.f.1": return symIcosF1();
    case "icos.f.3": return symIcosF3();
    case "icos.f.c": return symIcosFC(mesh);
    case "icos.v.1": return symIcosV1();
    case "dbg": return symDbg();
    default:
      throw new Error(`symmetry not supported: ${mesh.symmetry}`);
  }
}

function symIcosF1() {
  const meshes = meshList("I.f.1.", 20);

  for (let i = 0; i < 5; i++) {
    meshes[i]
      .rotate(Y, PI / 5 * 2 * i, WORLD);
    meshes[i + 5]
      .rotate(R, -PI / 5 * 2, WORLD)
      .rotate(Y, PI / 5 * 2 * i, WORLD);
    meshes[i + 10]
      .rotate(R, -PI / 5 * 2, WORLD)
      .rotate(X, PI, WORLD)
      .rotate(Y, PI / 5 * (i * 2 + 1), WORLD);
    meshes[i + 15]
      .rotate(X, PI, WORLD)
      .rotate(Y, PI / 5 * (i * 2 + 1), WORLD);
  }

  return meshes;
}

function symIcosF3() {
  const meshes = meshList("I.f.3.", 20 * 3);

  for (let i = 0; i < 5; i++) {
    for (let j = 0; j < 3; j++) {
      const k = j * 20 + i;

      Array(4).fill()
        .map((_, i) => i * 5)
        .forEach(x => meshes[k + x].rotate(O, PI / 3 * 2 * j, WORLD));

      meshes[k]
        .rotate(Y, PI / 5 * 2 * i, WORLD);
      meshes[k + 5]
        .rotate(R, -PI / 5 * 2, WORLD)
        .rotate(Y, PI / 5 * 2 * i, WORLD);
      meshes[k + 10]
        .rotate(R, -PI / 5 * 2, WORLD)
        .rotate(X, PI, WORLD)
        .rotate(Y, PI / 5 * (i * 2 + 1), WORLD);
      meshes[k + 15]
        .rotate(X, PI, WORLD)
        .rotate(Y, PI / 5 * (i * 2 + 1), WORLD);
    }
  }

  return meshes;
}

function symIcosFC(mesh) {
  const [x, y, z] = mesh.positions;

  const pos = (t, r) => {
    const v = new Vector3(x(t), y(t), z(t));
    const q = Quaternion.RotationAxis(O, PI / 3 * 2 * r);
    return v.rotateByQuaternionToRef(q, v);
  }

  for (let i = 1; i < 3; i++) {
    'xyz'.split('').forEach(axis => {
      mesh.positions.push(t => pos(t, i)[axis]);
    })
  }

  mesh.indices = [0, 2, 1];

  return symIcosF1();
}

function symIcosV1() {
  const meshes = meshList("I.v.1.", 12)

  for (let i = 0; i < 5; i++) {
    meshes[i + 1]
      .rotate(R, -PI / 5 * 2, WORLD)
      .rotate(Y, PI / 5 * 2 * i, WORLD);
    meshes[i + 6]
      .rotate(Y, PI / 5, WORLD)
      .rotate(X, PI, WORLD)
      .rotate(R, -PI / 5 * 2, WORLD)
      .rotate(Y, PI / 5 * 2 * i, WORLD);
  }

  meshes[11]
    .rotate(Y, PI / 5, WORLD)
    .rotate(X, PI, WORLD);

  return meshes;
}

function symDbg() {
  return [
    MeshBuilder.CreateSphere(`dbg.${Math.random()}`, { diameter: 0.05 }),
  ];
}


function meshList(prefix, count) {
  return Array(count)
    .fill()
    .map((_, i) => new Mesh(`${prefix}${i}`));
}
