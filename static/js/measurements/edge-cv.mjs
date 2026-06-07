import { coefficientOfVariation } from "../math/stats.mjs";

const { PI } = Math;

export function computeHexFaceEdgeCv({
  meshF3,
  meshFC,
  params,
  axis,
  Quaternion,
  Vector3,
}) {
  return computeHexFaceEdgeMetrics({
    meshF3,
    meshFC,
    params,
    axis,
    Quaternion,
    Vector3,
  }).cv;
}

export function computeHexFaceEdgeMetrics({
  meshF3,
  meshFC,
  params,
  axis,
  Quaternion,
  Vector3,
}) {
  if (!meshF3 || meshF3.symmetry !== "icos.f.3") {
    return { cv: null, mm: null, lengths: [], reason: "missing-or-wrong-f3-mesh" };
  }
  if (!meshFC || meshFC.symmetry !== "icos.f.c") {
    return { cv: null, mm: null, lengths: [], reason: "missing-or-wrong-fc-mesh" };
  }

  const patch = buildLocalHexPatch({
    meshF3,
    meshFC,
    params,
    axis,
    Quaternion,
    Vector3,
  });
  const boundaryEdges = findBoundaryEdges(patch.triangles);
  const isEndpoint = isNearEndpoint(params);
  if (isEndpoint && (boundaryEdges.length === 3 || boundaryEdges.length === 8)) {
    return {
      cv: 1,
      mm: 1,
      lengths: [],
      reason: "degenerate-hex",
    };
  }
  if (boundaryEdges.length !== 6) {
    return {
      cv: null,
      mm: null,
      lengths: [],
      reason: `expected-6-boundary-edges-got-${boundaryEdges.length}`,
    };
  }

  const lengths = boundaryEdges.map(([a, b]) => Vector3.Distance(patch.vertices[a], patch.vertices[b]));
  const mm = maxMinRatio(lengths);

  return {
    cv: coefficientOfVariation(lengths),
    mm,
    lengths,
    reason: null,
  };
}

function maxMinRatio(values) {
  if (!values.length) return 0;
  const max = Math.max(...values);
  if (max === 0) return 0;
  const min = Math.min(...values);
  return (max - min) / max;
}

function isNearEndpoint(params) {
  if (!params.length) return false;
  const t = params[0];
  const eps = 1e-9;
  return t <= eps || t >= 1 - eps;
}

function buildLocalHexPatch({
  meshF3,
  meshFC,
  params,
  axis,
  Quaternion,
  Vector3,
}) {
  const triangles = [];
  const vertices = [];

  const addRotatedTriangles = (baseVertices, baseTriangles) => {
    for (let r = 0; r < 3; r++) {
      const q = Quaternion.RotationAxis(axis, PI * 2 / 3 * r);
      const offset = vertices.length;

      baseVertices.forEach(vertex => {
        const rotated = new Vector3(0, 0, 0);
        vertex.rotateByQuaternionToRef(q, rotated);
        vertices.push(rotated);
      });

      baseTriangles.forEach(([a, b, c]) => {
        triangles.push([a + offset, b + offset, c + offset]);
      });
    }
  };

  const f3Vertices = positionsToVertices(meshF3.positions, params, Vector3);
  const f3Triangles = indicesToTriangles(meshF3.indices);
  addRotatedTriangles(f3Vertices, f3Triangles);

  const fcSeed = positionsToVertices(meshFC.positions, params, Vector3);
  let fcVertices;
  if (fcSeed.length === 1) {
    fcVertices = [0, 1, 2].map(r => {
      const q = Quaternion.RotationAxis(axis, PI * 2 / 3 * r);
      const rotated = new Vector3(0, 0, 0);
      fcSeed[0].rotateByQuaternionToRef(q, rotated);
      return rotated;
    });
  } else if (fcSeed.length === 3) {
    // `symIcosFC` mutates the source mesh to include these 3 vertices already.
    fcVertices = fcSeed;
  } else {
    return { vertices: [], triangles: [] };
  }
  // `icos.f.c` defines one local center triangle per face; do not rotate it again.
  const fcOffset = vertices.length;
  fcVertices.forEach(vertex => vertices.push(vertex));
  triangles.push([fcOffset + 0, fcOffset + 2, fcOffset + 1]);

  return weldVertices(vertices, triangles);
}

function positionsToVertices(positionFns, params, Vector3) {
  const coords = positionFns.map(fn => fn.apply(null, params));
  const vertices = [];

  for (let i = 0; i < coords.length; i += 3) {
    vertices.push(new Vector3(coords[i], coords[i + 1], coords[i + 2]));
  }

  return vertices;
}

function indicesToTriangles(indices) {
  const triangles = [];
  for (let i = 0; i < indices.length; i += 3) {
    triangles.push([indices[i], indices[i + 1], indices[i + 2]]);
  }
  return triangles;
}

function weldVertices(vertices, triangles) {
  const keyToIndex = new Map();
  const remap = [];
  const weldedVertices = [];

  vertices.forEach((vertex, i) => {
    const key = vertexKey(vertex);
    let index = keyToIndex.get(key);
    if (index === undefined) {
      index = weldedVertices.length;
      keyToIndex.set(key, index);
      weldedVertices.push(vertex);
    }
    remap[i] = index;
  });

  return {
    vertices: weldedVertices,
    triangles: triangles.map(([a, b, c]) => [remap[a], remap[b], remap[c]]),
  };
}

function vertexKey(vertex) {
  const eps = 1e-6;
  const q = value => Math.round(value / eps) * eps;
  return `${q(vertex.x)}:${q(vertex.y)}:${q(vertex.z)}`;
}

function findBoundaryEdges(triangles) {
  const edgeUse = new Map();

  triangles.forEach(([a, b, c]) => {
    [[a, b], [b, c], [c, a]].forEach(([u, v]) => {
      const edge = u < v ? [u, v] : [v, u];
      const key = `${edge[0]}:${edge[1]}`;
      edgeUse.set(key, (edgeUse.get(key) ?? 0) + 1);
    });
  });

  return Array.from(edgeUse.entries())
    .filter(([, count]) => count === 1)
    .map(([key]) => key.split(":").map(Number));
}
