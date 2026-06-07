# Icos

An interactive visualizer for Goldberg polyhedra on the sphere, exploring degrees of freedom under strict symmetry and
geometric constraints.

## Goal

The goal of this project is to visualize degrees of freedom in various (Goldberg) polyhedra while preserving key
properties:

1. **Icosahedral symmetry**: Preserves full 3D icosahedral symmetry.
2. **120° rotational symmetry**: Rotational symmetry around a single icosahedral face.
3. **"Left-right" symmetry**: Reflection/mirror symmetry around a single icosahedral face.
4. **"On-sphere" (inscribable)**: Vertices are constrained to lie on the unit sphere (distance from origin = 1),
   enforced by using a spherical coordinate system.

Under these constraints, the project investigates how deforming the polyhedra affects properties like edge-length
equality (equilateral faces) and face planarity.

## Current Polyhedra

- **(1, 0) Dodecahedron**: The base case. Has 0 degrees of freedom. Used for layout debugging.
- **(1, 1) Truncated Icosahedron**: Has 1 degree of freedom ($t_1$, defining the amount of truncation).
- **(2, 0) Goldberg**: Has 1 degree of freedom ($t_1$). Setting it to `~0.42` results in an equilateral, planar,
  on-sphere Goldberg polyhedron.

---

## TODO / Next Steps

- [ ] **Implement Goldberg (2, 2) Subdivision**.
