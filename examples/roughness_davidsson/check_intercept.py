#!/usr/bin/env python

import numpy

import kalast

# checks if point is lying on triangle 3d space "above" and "below" defined by its edges boundaries
p = numpy.array([0.0, 0.0, 2.0])
a = numpy.array([0.0, 0.0, 0.0])
b = numpy.array([1.0, 0.0, 0.0])
c = numpy.array([0.0, 1.0, 0.0])
x = kalast.mesh.is_point_in_or_on_triangle(p, a, b, c)
assert x

# test basic ray interception with plane, notice that ray is infinite,
# so length is not considered for interception, it is working with unit vector
p = numpy.array([0.0, 0.0, 1.0])
u = numpy.array([0.0, 0.0, -1.0])
a = numpy.array([0.0, 0.0, 0.0])
n = numpy.array([0.0, 0.0, 1.0])
x = kalast.mesh.intersect_plane(p, u, a, n)
assert (x == numpy.array([0.0, 0.0, 0.0])).all()

# ray interception means it is required to be facing the direction of the plane,
# here it fails cus looking opposite direction
# line interception would have work, but this is ray interception
p = numpy.array([0.0, 0.0, 2.0])
u = numpy.array([0.0, 0.0, 1.0])
a = numpy.array([0.0, 0.0, 0.0])
n = numpy.array([0.0, 0.0, 1.0])
x = kalast.mesh.intersect_plane(p, u, a, n)
assert x is None

# this results in same as plane interception that worked before
# it just contains extra checks lying on triangle 3d space boundaries
p = numpy.array([0.0, 0.0, 1.0])
u = numpy.array([0.0, 0.0, -1.0])
a = numpy.array([0.0, 0.0, 0.0])
b = numpy.array([1.0, 0.0, 0.0])
c = numpy.array([0.0, 1.0, 0.0])
n = numpy.array([0.0, 0.0, 1.0])
x = kalast.mesh.intersect_triangle(p, u, a, b, c, n)
assert (x == numpy.array([0.0, 0.0, 0.0])).all()

# for example here it succeed for plane but not for triangle cus the point is outside the triangle 3d space boundaries
p = numpy.array([2.0, 0.0, 1.0])
u = numpy.array([0.0, 0.0, -1.0])
a = numpy.array([0.0, 0.0, 0.0])
n = numpy.array([0.0, 0.0, 1.0])
x = kalast.mesh.intersect_plane(p, u, a, n)
assert (x == numpy.array([2.0, 0.0, 0.0])).all()

b = numpy.array([1.0, 0.0, 0.0])
c = numpy.array([0.0, 1.0, 0.0])
x = kalast.mesh.intersect_triangle(p, u, a, b, c, n)
assert x is None

# some checks on a cube
mesh = kalast.mesh.Mesh("res/cube.obj", lambda x: x)
assert len(mesh.vertices) == 8
assert mesh.indices.size == 36
assert len(mesh.facets) == 12
assert len(mesh._vertices_before_flatten) == 0

# indices of the vertices of selected facets
assert mesh.get_indices_facet(0) == [0, 1, 2]
assert mesh.get_indices_facet(1) == [1, 3, 4]
assert mesh.get_indices_facet(11) == [0, 2, 7]

[a, b, c] = mesh.get_positions_facet(0)
assert (a == numpy.array([-1.0, 1.0, 1.0])).all()
assert (b == numpy.array([1.0, -1.0, 1.0])).all()
assert (c == numpy.array([1.0, 1.0, 1.0])).all()

[a, b, c] = mesh.get_positions_facet(1)
assert (a == numpy.array([1.0, -1.0, 1.0])).all()
assert (b == numpy.array([-1.0, -1.0, -1.0])).all()
assert (c == numpy.array([1.0, -1.0, -1.0])).all()

[a, b, c] = mesh.get_positions_facet(11)
assert (a == numpy.array([-1.0, 1.0, 1.0])).all()
assert (b == numpy.array([1.0, 1.0, 1.0])).all()
assert (c == numpy.array([1.0, 1.0, -1.0])).all()

assert (mesh.facets[0].n == numpy.array([0.0, 0.0, 1.0])).all()
assert (mesh.facets[1].n == numpy.array([0.0, -1.0, 0.0])).all()
assert (mesh.facets[11].n == numpy.array([0.0, 1.0, 0.0])).all()

# flatten mesh and re-test vertices positions
mesh.flatten()

assert len(mesh.vertices) == 36
assert mesh.indices.size == 36
assert len(mesh.facets) == 12
assert len(mesh._vertices_before_flatten) == 8

[a, b, c] = mesh.get_positions_facet(0)
assert (a == numpy.array([-1.0, 1.0, 1.0])).all()
assert (b == numpy.array([1.0, -1.0, 1.0])).all()
assert (c == numpy.array([1.0, 1.0, 1.0])).all()

[a, b, c] = mesh.get_positions_facet(1)
assert (a == numpy.array([1.0, -1.0, 1.0])).all()
assert (b == numpy.array([-1.0, -1.0, -1.0])).all()
assert (c == numpy.array([1.0, -1.0, -1.0])).all()

[a, b, c] = mesh.get_positions_facet(11)
assert (a == numpy.array([-1.0, 1.0, 1.0])).all()
assert (b == numpy.array([1.0, 1.0, 1.0])).all()
assert (c == numpy.array([1.0, 1.0, -1.0])).all()

[a, b, c] = mesh.get_normals_facet(0)
assert (a == numpy.array([0.0, 0.0, 1.0])).all()
assert (b == numpy.array([0.0, 0.0, 1.0])).all()
assert (c == numpy.array([0.0, 0.0, 1.0])).all()

[a, b, c] = mesh.get_normals_facet(1)
assert (a == numpy.array([0.0, -1.0, 0.0])).all()
assert (b == numpy.array([0.0, -1.0, 0.0])).all()
assert (c == numpy.array([0.0, -1.0, 0.0])).all()

[a, b, c] = mesh.get_normals_facet(11)
assert (a == numpy.array([0.0, 1.0, 0.0])).all()
assert (b == numpy.array([0.0, 1.0, 0.0])).all()
assert (c == numpy.array([0.0, 1.0, 0.0])).all()

# re-smooth and check it's back to origin
mesh.smoothen()

assert len(mesh.vertices) == 8
assert mesh.indices.size == 36
assert len(mesh.facets) == 12
assert len(mesh._vertices_before_flatten) == 0

[a, b, c] = mesh.get_positions_facet(0)
assert (a == numpy.array([-1.0, 1.0, 1.0])).all()
assert (b == numpy.array([1.0, -1.0, 1.0])).all()
assert (c == numpy.array([1.0, 1.0, 1.0])).all()

[a, b, c] = mesh.get_positions_facet(1)
assert (a == numpy.array([1.0, -1.0, 1.0])).all()
assert (b == numpy.array([-1.0, -1.0, -1.0])).all()
assert (c == numpy.array([1.0, -1.0, -1.0])).all()

[a, b, c] = mesh.get_positions_facet(11)
assert (a == numpy.array([-1.0, 1.0, 1.0])).all()
assert (b == numpy.array([1.0, 1.0, 1.0])).all()
assert (c == numpy.array([1.0, 1.0, -1.0])).all()

# some ray interceptions
p = numpy.array([0.0, 0.0, 10.0])
u = numpy.array([0.0, 0.0, -1.0])
r = mesh.intersect(p, u)
assert r[0] == 0
assert numpy.all(r[1] - numpy.array([0.0, 0.0, 1.0]) < 1e-7)

p = numpy.array([0.0, 0.0, -10.0])
u = numpy.array([0.0, 0.0, 1.0])
r = mesh.intersect(p, u)
assert r[0] == 3
assert numpy.all(r[1] - numpy.array([0.0, 0.0, -1.0]) < 1e-7)

p = numpy.array([-10.0, 0.0, 0.0])
u = numpy.array([1.0, 0.0, 0.0])
r = mesh.intersect(p, u)
assert r[0] == 2
assert numpy.all(r[1] - numpy.array([-1.0, 0.0, 0.0]) < 1e-7)

p = numpy.array([10.0, 0.0, 0.0])
u = numpy.array([-1.0, 0.0, 0.0])
r = mesh.intersect(p, u)
assert r[0] == 4
assert numpy.all(r[1] - numpy.array([1.0, 0.0, 0.0]) < 1e-7)

# now working on full surface by loading shape model, a plane with a crater for davidsson roughness test cases
# The plane has 1024 square facets that have been triangulated, so 2048 triangle facets.
# The dimensions are from -0.5 to 0.5 in X and Y axes, on 0 Z.
# The crater has a radius of size 0.437 to match a crater coverage of 60%.
# The crater has negative Z values and depth of crater is same as crater radius.
mesh = kalast.mesh.Mesh("res/plane_crater_1024-5000_h=0.437.obj", lambda x: x)

# 3267 / 3 = 1089 vertices
# 6144 / 3 = 2048 facets (triangles)
assert len(mesh.vertices) == 1089
assert mesh.indices.size == 6144
assert len(mesh.facets) == 2048
assert len(mesh._vertices_before_flatten) == 0

# indices of the vertices of selected facets
assert mesh.get_indices_facet(0) == [0, 1, 2]
assert mesh.get_indices_facet(1) == [3, 4, 0]
assert mesh.get_indices_facet(10) == [21, 22, 19]
assert mesh.get_indices_facet(2047) == [1086, 1088, 1087]

# positions of the vertices of selected facets
[a, b, c] = mesh.get_positions_facet(0)
assert (a == numpy.array([-0.46875, -0.5, 0])).all()
assert (b == numpy.array([-0.5, -0.46875, 0])).all()
assert (c == numpy.array([-0.5, -0.5, 0])).all()

[a, b, c] = mesh.get_positions_facet(1)
assert (a == numpy.array([-0.4375, -0.5, 0])).all()
assert (b == numpy.array([-0.46875, -0.46875, 0])).all()
assert (c == numpy.array([-0.46875, -0.5, 0])).all()

[a, b, c] = mesh.get_positions_facet(2)
assert (a == numpy.array([-0.40625, -0.5, 0])).all()
assert (b == numpy.array([-0.4375, -0.46875, 0])).all()
assert (c == numpy.array([-0.4375, -0.5, 0])).all()

[a, b, c] = mesh.get_positions_facet(10)
assert (a == numpy.array([-0.15625, -0.5, 0])).all()
assert (b == numpy.array([-0.1875, -0.46875, 0])).all()
assert (c == numpy.array([-0.1875, -0.5, 0])).all()

[a, b, c] = mesh.get_positions_facet(2047)
assert (a == numpy.array([0.5, 0.46875, 0.0])).all()
assert (b == numpy.array([0.5, 0.5, 0.0])).all()
assert (c == numpy.array([0.46875, 0.5, 0.0])).all()

# top-down ray to intercept with bottom depth inside of crater. True for argument exit_first is to intercept first facet found, False
# is to respect Z-buffer and find closest to ray facet, False is default.
p = numpy.array([0.0, 0.0, 0.0])
u = numpy.array([0.0, 0.0, -1.0])
r = mesh.intersect(p, u)
assert r[0] == 496
assert numpy.all(r[1] - numpy.array([0.0, 0.0, -0.437]) < 1e-7)

# specific case for parallel ray to normals, it does not intercept triangles from the 0 dot product in denominator in
# formula, hence it only intercept with the first facet that is facing, which is the other side of the crater
# It does intercept cus opposite facet is actually a bit tilted.
p = numpy.array([1.0, 0.0, 0.0])
u = numpy.array([-1.0, 0.0, 0.0])
r = mesh.intersect(p, u)
assert r[0] == 1506
assert numpy.all(r[1] - numpy.array([-0.43749988, 0.0, 0.0]) < 1e-8)

# top to "down left" ray with 45° angle to intercept with "bottom left" part inside of crater
p = numpy.array([0.0, 0.0, 0.0])
u = numpy.array([-1.0, 0.0, -1.0])
r = mesh.intersect(p, u)
assert r[0] == 1510
assert numpy.all((r[1] - numpy.array([-0.30834377, 0.0, -0.30834377])) < 1e-8)

# top to "down left" ray with 45° angle to intercept with "bottom left" part inside of crater, the ray is casted 0.4
# higher on Z axis, intercepted triangle is expected almost out of crater
p = numpy.array([0.0, 0.0, 0.4])
u = numpy.array([-1.0, 0.0, -1.0])
r = mesh.intersect(p, u)
assert r[0] == 1506
assert numpy.all((r[1] - numpy.array([-0.4313387, 0.0, -0.0313387])) < 1e-8)

# same but casted at 0.5 +Z so expected to intercept on border on -X of mesh
p = numpy.array([0.0, 0.0, 0.5])
u = numpy.array([-1.0, 0.0, -1])
r = mesh.intersect(p, u)
assert r[0] == 480
assert numpy.all((r[1] - numpy.array([-0.5, 0.0, 0.0])) < 1e-8)

# same but at 0.51 +Z so expected to miss
p = numpy.array([0.0, 0.0, 0.6])
u = numpy.array([-1.0, 0.0, -1.0])
r = mesh.intersect(p, u)
assert r is None

# casted at 1+X and 1 +Z so it should almost reproduced the first case with 45° angle casted from 0.0 +Z
p = numpy.array([1.0, 0.0, 1.0])
u = numpy.array([-1.0, 0.0, -1.0])
r = mesh.intersect(p, u)
assert r[0] == 1542
assert numpy.all((r[1] - numpy.array([-0.30834365, 0.0, -0.30834365])) < 1e-8)

# so yeah, it's not exactly the same but really almost, i guess it's f32 float precision issue here
# and f64 would give the same
assert (
    numpy.linalg.norm(
        numpy.array([-0.30834377, 0.0, -0.30834377])  # 1st case
        - numpy.array([-0.30834365, 0.0, -0.30834365])  # 2nd case
    )
    < 1e-6
)

# from under crater trying to intercept, but interception respect facing ray triangle (respecting back-face culling) so
# no interception is expected here
p = numpy.array([0.0, 0.0, -10.0])
u = numpy.array([0.0, 0.0, 1.0])
r = mesh.intersect(p, u)
assert r is None

# trying to intercept from under crater but directed on X/Y plane to the side, back-face culling should still catch
# some facets
p = numpy.array([1.0, 0.0, -0.1])
u = numpy.array([-1.0, 0.0, 0.0])
r = mesh.intersect(p, u)
assert r[0] == 1506
assert numpy.all((r[1] - numpy.array([-0.41783881, 0.0, -0.1])) < 1e-8)

# same but at 0.4 -Z
p = numpy.array([1.0, 0.0, -0.4])
u = numpy.array([-1.0, 0.0, 0.0])
r = mesh.intersect(p, u)
assert r[0] == 1514
assert numpy.all((r[1] - numpy.array([-0.17184079, 0.0, -0.4])) < 1e-8)

# same but at most bottom depth height
p = numpy.array([1.0, 0.0, -0.437])
u = numpy.array([-1.0, 0.0, 0.0])
r = mesh.intersect(p, u)
assert r[0] == 1519
assert numpy.all((r[1] - numpy.array([0.0, 0.0, -0.437])) < 1e-8)

# same but slightly missing
p = numpy.array([1.0, 0.0, -0.438])
u = numpy.array([-1.0, 0.0, 0.0])
r = mesh.intersect(p, u)
assert r is None
