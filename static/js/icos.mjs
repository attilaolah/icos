import { computeHexFaceEdgeMetrics } from "./measurements/edge-cv.mjs";

const { PI } = Math;
const { ArcRotateCamera, Color3, Color4, Engine, HemisphericLight, Mesh, MeshBuilder, Quaternion, Scene, Space, StandardMaterial, Vector3, VertexData } = BABYLON;
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
  camera.wheelPrecision = 12; // Decrease zoom speed (1/4 as sensitive)
  camera.attachControl(canvas, true);

  const geometry = await (await req).json();
  const paramsState = geometry.params.map(pval);
  const meshData = geometry.meshes.map(data => ({
    ...data,
    positions: data.positions.map(pos => pfun(pos, paramsState.length)),
  }));

  const updates = meshData.map(data => {
    const meshes = symmetry(data);

    const vd = new VertexData();
    vd.indices = data.indices;

    let dirty = true;
    return paramsChanged => {
      if (paramsChanged) dirty = true;
      if (!dirty) return;
      dirty = false;

      vd.positions = data.positions.map(fn => fn.apply(null, paramsState));
      meshes.forEach(m => {
        vd.applyToMesh(m);
      });
    };
  });

  const paramsContainer = document.body.querySelector(".params");
  Array.from(paramsContainer.children).forEach(elem => elem.remove());
  const controls = geometry.params.map((t, i) => {
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

    return { label, input, output };
  });
  paramsContainer.append(...controls.map(control => control.label));

  const measurementsSection = document.getElementById("measurements-section");
  const cvValue = document.getElementById("measurement-e-cv");
  const mmValue = document.getElementById("measurement-e-mm");
  const cvMeshF3 = meshData.find(mesh => mesh.symmetry === "icos.f.3");
  const cvMeshFC = meshData.find(mesh => mesh.symmetry === "icos.f.c");
  const measurementsVisible = shape === "goldberg.1.1";
  let measurementDirty = true;
  measurementsSection.hidden = !measurementsVisible;
  const updateMarkers = setupGoldberg22PointMarkers({
    shape,
    scene,
    meshData,
    params: paramsState,
  });

  engine.runRenderLoop(() => {
    let paramsChanged = false;

    controls.forEach((control, i) => {
      const current = paramsState[i];
      const next = parseFloat(control.input.value);
      if (next !== current) {
        paramsState[i] = next;
        control.output.value = next.toFixed(2);
        paramsChanged = true;
      }
    });

    updates.forEach(fn => fn(paramsChanged));
    updateMarkers(paramsChanged);
    if (paramsChanged) measurementDirty = true;
    if (measurementDirty) {
      measurementDirty = false;
      updateMeasurements({
        measurementsVisible,
        cvValue,
        mmValue,
        cvMeshF3,
        cvMeshFC,
        params: paramsState,
        shape,
      });
    }

    scene.render();
  });

  resize = window.addEventListener("resize", function() {
    engine.resize();
  });
}

function updateMeasurements({ measurementsVisible, cvValue, mmValue, cvMeshF3, cvMeshFC, params, shape }) {
  if (!measurementsVisible) {
    cvValue.innerText = "0.000000";
    mmValue.innerText = "0.000000";
    return;
  }

  if (shape !== "goldberg.1.1") {
    cvValue.innerText = "0.000000";
    mmValue.innerText = "0.000000";
    return;
  }

  const { cv, mm } = computeHexFaceEdgeMetrics({
    meshF3: cvMeshF3,
    meshFC: cvMeshFC,
    params,
    axis: O,
    Quaternion,
    Vector3,
  });

  if (cv === null) {
    cvValue.innerText = "0.000000";
    mmValue.innerText = "0.000000";
    return;
  }

  cvValue.innerText = cv.toFixed(6);
  mmValue.innerText = mm.toFixed(6);
}

function setupGoldberg22PointMarkers({ shape, scene, meshData, params }) {
  if (shape !== "goldberg.2.2") return () => {};

  const localOuterSeeds = meshData.filter(mesh =>
    mesh.symmetry === "icos.f.3" && mesh.indices.length === 3 && mesh.positions.length === 9);
  if (localOuterSeeds.length < 3) return () => {};

  const [seedABF, seedBFC, seedBCD] = localOuterSeeds;
  const radius = 0.0175;
  const defs = [
    { label: "A", color: new Color3(1.0, 0.2, 0.2) },
    { label: "B", color: new Color3(1.0, 0.55, 0.2) },
    { label: "C", color: new Color3(1.0, 0.9, 0.2) },
    { label: "D", color: new Color3(0.2, 0.9, 0.2) },
    { label: "E", color: new Color3(0.2, 0.8, 1.0) },
    { label: "F", color: new Color3(0.8, 0.3, 1.0) },
  ];

  const spheres = defs.map(def => {
    const sphere = MeshBuilder.CreateSphere(`dbg.hex.${def.label}`, { diameter: radius * 2 }, scene);
    const mat = new StandardMaterial(`dbg.hex.${def.label}.mat`, scene);
    mat.diffuseColor = def.color;
    mat.emissiveColor = def.color.scale(0.45);
    sphere.material = mat;
    return sphere;
  });

  const evalVertex = (mesh, vertex) => {
    const i = vertex * 3;
    return new Vector3(
      mesh.positions[i].apply(null, params),
      mesh.positions[i + 1].apply(null, params),
      mesh.positions[i + 2].apply(null, params),
    );
  };

  const pointFromNormSpherical = (thetaPi, phiPi) => {
    const theta = thetaPi * PI;
    const phi = phiPi * PI;
    const sinTheta = Math.sin(theta);
    return new Vector3(
      sinTheta * Math.cos(phi),
      Math.cos(theta),
      sinTheta * Math.sin(phi),
    );
  };

  const update = () => {
    const a = evalVertex(seedABF, 0);
    const b = evalVertex(seedABF, 1);
    const f = evalVertex(seedABF, 2);

    const c = evalVertex(seedBCD, 2);
    const d = evalVertex(seedBFC, 2);
    const e = pointFromNormSpherical(params[2], 2 / 5 - params[3]);

    [a, b, c, d, e, f].forEach((point, i) => {
      spheres[i].position.copyFrom(point);
    });
  };

  update();
  return paramsChanged => {
    if (!paramsChanged) return;
    update();
  };
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




function meshList(prefix, count) {
  return Array(count)
    .fill()
    .map((_, i) => new Mesh(`${prefix}${i}`));
}
