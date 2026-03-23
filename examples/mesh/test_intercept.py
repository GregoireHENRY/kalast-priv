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

# ray interception means it is required to be facing the direction of the plane
# it doesnt fail even if looking at opposite direction
# can check if facing plane separately
p = numpy.array([0.0, 0.0, 2.0])
u = numpy.array([0.0, 0.0, 1.0])
a = numpy.array([0.0, 0.0, 0.0])
n = numpy.array([0.0, 0.0, 1.0])
x = kalast.mesh.intersect_plane(p, u, a, n)
assert (x == numpy.array([0.0, 0.0, 0.0])).all()
assert not kalast.mesh.is_facing_plane(u, n)

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

# some checks with a cube
mesh = kalast.mesh.Mesh("res/cube.obj")

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

# top-down ray to intercept with bottom depth inside of crater. True for argument exit_first is to intercept first facet found, False
# is to respect Z-buffer and find closest to ray facet, False is default.
p = numpy.array([0.0, 0.0, 0.0])
u = numpy.array([0.0, 0.0, -1.0])
r = mesh.intersect(p, u)
assert r[0] == 496
assert numpy.all(r[1] - numpy.array([0.0, 0.0, -0.437]) < 1e-7)

# specific case for parallel ray to normals, it does not intercept triangles from the 0 dot product in denominator in
# formula, hence it only intercept with the first facet that is met
p = numpy.array([1.0, 0.0, 0.0])
u = numpy.array([-1.0, 0.0, 0.0])
r = mesh.intersect(p, u)
assert r[0] == 509
assert numpy.all(r[1] - numpy.array([0.4375, 0.0, 0.0]) < 1e-8)

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

# from under crater trying to intercept
# interception happens even if normals are not facing, same as test with
# the plane above, can check is_facing
p = numpy.array([0.0, 0.0, -10.0])
u = numpy.array([0.0, 0.0, 1.0])
r = mesh.intersect(p, u)
assert r[0] == 496
assert numpy.all((r[1] - numpy.array([-0.0, 0.0, -0.437])) < 1e-8)
assert not kalast.mesh.is_facing_plane(u, n)

# trying to intercept from under crater but directed on X/Y plane to the side
p = numpy.array([1.0, 0.0, -0.1])
u = numpy.array([-1.0, 0.0, 0.0])
r = mesh.intersect(p, u)
assert r[0] == 509
assert numpy.all((r[1] - numpy.array([0.4181, 0.0, -0.1])) < 1e-8)

# same but at 0.4 -Z
p = numpy.array([1.0, 0.0, -0.4])
u = numpy.array([-1.0, 0.0, 0.0])
r = mesh.intersect(p, u)
assert r[0] == 501
assert numpy.all((r[1] - numpy.array([0.175, 0.0, -0.4])) < 1e-8)

# same but at most bottom depth height
p = numpy.array([1.0, 0.0, -0.437])
u = numpy.array([-1.0, 0.0, 0.0])
r = mesh.intersect(p, u)
assert r[0] == 496
assert numpy.all((r[1] - numpy.array([0.0, 0.0, -0.437])) < 1e-8)

# same but slightly missing
p = numpy.array([1.0, 0.0, -0.438])
u = numpy.array([-1.0, 0.0, 0.0])
r = mesh.intersect(p, u)
assert r is None
