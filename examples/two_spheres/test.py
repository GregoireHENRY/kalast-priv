#!/usr/bin/env python

import numpy

import kalast


app = kalast.app.App()

app.simulation.sun = [0.0, 20.0, 0.0]

mesh = kalast.mesh.Mesh(path="res/ico3.obj")
# print(f"vertices: {len(mesh.vertices)}")
# print(f"facets: {len(mesh.facets)}")
mesh.flatten()

mat = numpy.eye(4, dtype=numpy.float32)
mat[:3, :3] *= numpy.eye(3) * 0.2
mat[0:3, 3] = [0.0, 5.0, 0.0]
app.simulation.add_body(mesh, mat=mat)

app.simulation.bodies[0].instance.mat[:3, 3] = [0.0, 10.0, 0.0]
