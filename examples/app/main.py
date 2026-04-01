#!/usr/bin/env python

import numpy

import kalast


app = kalast.app.App()

# app.config.shader_color_mode = 0
# app.config.shader_color = [1.0, 1.0, 1.0, 0.0]
# app.config.debug_depth_show = True
app.config.debug_light_cube_show = True

app.config.ambient_strength = 0.002
app.config.render_back_face = True

# Set camera pos/up
# Different methods to set dir
app.simulation.camera.pos = [0.0, 5.0, 0.0]
app.simulation.camera.up = [0.0, 0.0, 1.0]
app.simulation.camera.look_anchor()

# app.simulation.camera.projection.zfar = 100.0
# app.simulation.camera.projection.side = 1.0
# app.simulation.camera.projection.set_orthographic()
# app.simulation.camera.projection.fovy = 45.0 * kalast.util.RPD

app.simulation.sun = [0.0, 20.0, 6.0]

mesh = kalast.mesh.Mesh(path="res/plane_crater_1024-5000_h=0.437.obj")
# print(len(mesh.vertices))
# print(len(mesh.facets))
mesh.flatten()
mat = numpy.eye(4, dtype=numpy.float32)
# mat[0:3, 3] = [2.5, 0.0, 0.0]
app.simulation.add_body(mesh, mat=mat)


def tick(sim):
    # print(f"#{sim.state.iteration}")

    pass


app.tick = tick
app.start()
