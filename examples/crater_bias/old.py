#!/usr/bin/env python

import numpy

import kalast


app = kalast.app.App()

app.config.global_color_mode = 0
# app.config.global_color = [1.0, 1.0, 1.0, 0.0]
# app.config.debug_depth_show = True
app.config.debug_light_cube_show = True

# app.config.render_back_face = True

# app.config.ambient_strength = 0.005

# app.config.shaow_resolution = 8192

# You want this value as low as possible until shadow acne appears.
# Value too large will create petter-panning effect.
# Increase until you observe petter-panning.
app.config.shadow_bias_scale = 1e-6

# Increase to remove global shadow acne artifact and erroneous self-shadowing.
app.config.shadow_bias_minimum = 1e-6

# Ideally keep this value to 0.0 or as close as possible to 0.0.
# But you can increase this value to allow smaller bias before acne appears.
# Value too large will cause jagged/sawtooth shadow terminator.
app.config.shadow_normal_offset_scale = 3e-4

# app.config.shadow_pcf = 1

app.config.light_side = 0.5
app.config.light_znear = 10.0
app.config.light_zfar = 30.0

# app.config.light_up = [0.0, 1.0, 0.0]
app.simulation.sun = [0.0, 20.0, 1.0]

# Set camera pos/up
# Different methods to set dir
app.simulation.camera.pos = [0.0, 5.0, 0.0]
app.simulation.camera.up = [0.0, 0.0, 1.0]
app.simulation.camera.look_anchor()

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
