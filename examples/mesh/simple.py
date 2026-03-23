#!/usr/bin/env python

# import numpy

import kalast


# create a vertex
v = kalast.mesh.Vertex(
    pos=[0.0, 0.0, 0.0],
    normal=[0.0, 0.0, 1.0],
)

# create a facet
# the plane of the facet is defined by a point (its center) and a normal
f = kalast.mesh.Facet(
    pos=[0.0, 0.0, 0.0],
    normal=[0.0, 0.0, 1.0],
    area=0.05,
)

# create a mesh from vertices and indices
m = kalast.mesh.Mesh(
    vertices=[
        kalast.mesh.Vertex(
            pos=[0.0, 0.0, 0.0],
            normal=[1.0, 1.0, 1.0],
        ),
        kalast.mesh.Vertex(
            pos=[3.0, 3.0, 3.0],
            normal=[4.0, 4.0, 4.0],
        ),
        kalast.mesh.Vertex(
            pos=[5.0, 5.0, 5.0],
            normal=[6.0, 6.0, 6.0],
        ),
    ],
)

# can change x composant of position of 1st vertex
m.vertices[0].pos[0] = 1.0

# can access all positions
m.positions

# all normals
m.normals

# all facets
m.facets

# all indices
m.indices

# load a mesh wavefront file
m = kalast.mesh.Mesh.load("res/cube.obj")