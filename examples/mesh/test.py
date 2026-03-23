#!/usr/bin/env python

import numpy

import kalast


# some checks loading a cube
# mesh = kalast.mesh.Mesh("res/cube.obj", lambda x: x)
mesh = kalast.mesh.Mesh("res/cube.obj")
assert len(mesh.vertices) == 8
assert mesh.indices.size == 36
assert len(mesh.facets) == 12
assert len(mesh._vertices_before_flatten) == 0

# alternative access for array based view
# by default a simple cube obj file is just vertices and indices of facets
# facets are initialized at loading for flattening and calculation
# other fields are usually zeros except is specifically mentionned
assert mesh.positions.shape == (8, 3)
assert mesh.textures.shape == (8, 2)
assert mesh.normals.shape == (8, 3)
assert mesh.tangents.shape == (8, 3)
assert mesh.bitangents.shape == (8, 3)
assert mesh.colors.shape == (8, 3)
assert mesh.color_modes.shape == (8,)

# indices of the vertices of selected facets
assert mesh.get_facet_indices(0) == [0, 1, 2]
assert mesh.get_facet_indices(1) == [1, 3, 4]
assert mesh.get_facet_indices(11) == [0, 2, 7]

[a, b, c] = mesh.get_facet_positions(0)
assert (a == numpy.array([-1.0, 1.0, 1.0])).all()
assert (b == numpy.array([1.0, -1.0, 1.0])).all()
assert (c == numpy.array([1.0, 1.0, 1.0])).all()

# These two methods ask the same thing.
assert (mesh.get_facet_positions(0) == mesh.positions[mesh.get_facet_indices(0)]).all()

[a, b, c] = mesh.get_facet_positions(1)
assert (a == numpy.array([1.0, -1.0, 1.0])).all()
assert (b == numpy.array([-1.0, -1.0, -1.0])).all()
assert (c == numpy.array([1.0, -1.0, -1.0])).all()

[a, b, c] = mesh.get_facet_positions(11)
assert (a == numpy.array([-1.0, 1.0, 1.0])).all()
assert (b == numpy.array([1.0, 1.0, 1.0])).all()
assert (c == numpy.array([1.0, 1.0, -1.0])).all()

assert (mesh.facets[0].normal == numpy.array([0.0, 0.0, 1.0])).all()
assert (mesh.facets[1].normal == numpy.array([0.0, -1.0, 0.0])).all()
assert (mesh.facets[11].normal == numpy.array([0.0, 1.0, 0.0])).all()

# flatten mesh and re-test vertices positions
mesh.flatten()

assert len(mesh.vertices) == 36
assert mesh.indices.size == 36
assert len(mesh.facets) == 12
assert len(mesh._vertices_before_flatten) == 8

assert mesh.positions.shape == (36, 3)
assert mesh.normals.shape == (36, 3)

[a, b, c] = mesh.get_facet_positions(0)
assert (a == numpy.array([-1.0, 1.0, 1.0])).all()
assert (b == numpy.array([1.0, -1.0, 1.0])).all()
assert (c == numpy.array([1.0, 1.0, 1.0])).all()

[a, b, c] = mesh.get_facet_positions(1)
assert (a == numpy.array([1.0, -1.0, 1.0])).all()
assert (b == numpy.array([1.0, -1.0, 1.0])).all()
assert (c == numpy.array([-1.0, -1.0, -1.0])).all()

[a, b, c] = mesh.get_facet_positions(11)
assert (a == numpy.array([-1.0, 1.0, 1.0])).all()
assert (b == numpy.array([1.0, 1.0, 1.0])).all()
assert (c == numpy.array([-1.0, 1.0, -1.0])).all()

[a, b, c] = mesh.get_facet_normals(0)
assert (a == numpy.array([0.0, 0.0, 1.0])).all()
assert (b == numpy.array([0.0, 0.0, 1.0])).all()
assert (c == numpy.array([0.0, 0.0, 1.0])).all()

[a, b, c] = mesh.get_facet_normals(1)
assert (a == numpy.array([0.0, 0.0, 1.0])).all()
assert (b == numpy.array([0.0, -1.0, 0.0])).all()
assert (c == numpy.array([0.0, -1.0, 0.0])).all()

[a, b, c] = mesh.get_facet_normals(11)
assert (a == numpy.array([0.0, 0.0, 1.0])).all()
assert (b == numpy.array([0.0, 0.0, 1.0])).all()
assert (c == numpy.array([-1.0, 0.0, 0.0])).all()

# re-smooth and check it's back to origin
mesh.smoothen()

assert len(mesh.vertices) == 8
assert mesh.indices.size == 36
assert len(mesh.facets) == 12
assert len(mesh._vertices_before_flatten) == 0

[a, b, c] = mesh.get_facet_positions(0)
assert (a == numpy.array([-1.0, 1.0, 1.0])).all()
assert (b == numpy.array([1.0, -1.0, 1.0])).all()
assert (c == numpy.array([1.0, 1.0, 1.0])).all()

[a, b, c] = mesh.get_facet_positions(1)
assert (a == numpy.array([1.0, -1.0, 1.0])).all()
assert (b == numpy.array([-1.0, -1.0, -1.0])).all()
assert (c == numpy.array([1.0, -1.0, -1.0])).all()

[a, b, c] = mesh.get_facet_positions(11)
assert (a == numpy.array([-1.0, 1.0, 1.0])).all()
assert (b == numpy.array([1.0, 1.0, 1.0])).all()
assert (c == numpy.array([1.0, 1.0, -1.0])).all()

# now working on full surface by loading shape model, a plane with a crater for davidsson roughness test cases
# The plane has 1024 square facets that have been triangulated, so 2048 triangle facets.
# The dimensions are from -0.5 to 0.5 in X and Y axes, on 0 Z.
# The crater has a radius of size 0.437 to match a crater coverage of 60%.
# The crater has negative Z values and depth of crater is same as crater radius.
mesh = kalast.mesh.Mesh("res/plane_crater_1024-5000_h=0.437.obj")

# 3267 / 3 = 1089 vertices
# 6144 / 3 = 2048 facets (triangles)
assert len(mesh.vertices) == 1089
assert mesh.indices.size == 6144
assert len(mesh.facets) == 2048
assert len(mesh._vertices_before_flatten) == 0

# indices of the vertices of selected facets
assert mesh.get_facet_indices(0) == [0, 1, 2]
assert mesh.get_facet_indices(1) == [3, 4, 0]
assert mesh.get_facet_indices(10) == [21, 22, 19]
assert mesh.get_facet_indices(2047) == [1086, 1088, 1087]

# positions of the vertices of selected facets
[a, b, c] = mesh.get_facet_positions(0)
assert (a == numpy.array([-0.46875, -0.5, 0])).all()
assert (b == numpy.array([-0.5, -0.46875, 0])).all()
assert (c == numpy.array([-0.5, -0.5, 0])).all()

[a, b, c] = mesh.get_facet_positions(1)
assert (a == numpy.array([-0.4375, -0.5, 0])).all()
assert (b == numpy.array([-0.46875, -0.46875, 0])).all()
assert (c == numpy.array([-0.46875, -0.5, 0])).all()

[a, b, c] = mesh.get_facet_positions(2)
assert (a == numpy.array([-0.40625, -0.5, 0])).all()
assert (b == numpy.array([-0.4375, -0.46875, 0])).all()
assert (c == numpy.array([-0.4375, -0.5, 0])).all()

[a, b, c] = mesh.get_facet_positions(10)
assert (a == numpy.array([-0.15625, -0.5, 0])).all()
assert (b == numpy.array([-0.1875, -0.46875, 0])).all()
assert (c == numpy.array([-0.1875, -0.5, 0])).all()

[a, b, c] = mesh.get_facet_positions(2047)
assert (a == numpy.array([0.5, 0.46875, 0.0])).all()
assert (b == numpy.array([0.5, 0.5, 0.0])).all()
assert (c == numpy.array([0.46875, 0.5, 0.0])).all()