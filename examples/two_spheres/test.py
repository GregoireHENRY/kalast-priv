#!/usr/bin/env python

import numpy

import kalast


app = kalast.app.App()

app.simulation.sun = [0.0, 20.0, 0.0]

mat = numpy.eye(4, dtype=numpy.float32)
app.simulation.load_mesh(path="res/ico3.obj", mat=mat, flatten=True)

mesh = kalast.mesh.Mesh(path="res/ico3.obj")
# print(f"vertices: {len(mesh.vertices)}")
# print(f"facets: {len(mesh.facets)}")
mesh.flatten()

mat = numpy.eye(4, dtype=numpy.float32)
mat[:3, :3] *= numpy.eye(3) * 0.2
mat[0:3, 3] = [0.0, 5.0, 0.0]
app.simulation.add_mesh(mesh, mat=mat)


app.simulation.bodies[0].mat[:3, 3] = [0.0, 10.0, 0.0]
