#!/usr/bin/env python

import numpy

import kalast


app = kalast.app.App()

app.config.shader_color_mode = 0

# Set camera pos/up
# Different methods to set dir
app.simulation.camera.pos = [0.0, 5.0, 0.0]
app.simulation.camera.up = [0.0, 0.0, 1.0]
app.simulation.camera.look_anchor()
app.simulation.camera.projection.fovy = 45.0 * kalast.util.RPD

mesh = kalast.mesh.Mesh(path="res/cube.obj")
mesh.update_colors(mode=0, color=[1.0, 1.0, 1.0])
mat = numpy.eye(4, dtype=numpy.float32)
app.simulation.add_body(mesh, mat=mat)

def tick(sim):
    # print(f"#{sim.state.iteration}")

    pass


app.tick = tick
app.start()
