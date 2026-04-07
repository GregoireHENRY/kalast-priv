#!/usr/bin/env python

import numpy

import kalast


app = kalast.app.App()

app.config.global_color_mode = 0

app.config.debug_light_cube_show = True

# Ideally keep this value to 0.0 or as close as possible to 0.0.
# But you can increase this value to allow smaller bias before acne appears.
# Value too large will cause jagged/sawtooth shadow terminator.
app.config.shadow_normal_offset_scale = 2e-4

# You want this value as low as possible until shadow acne appears.
# Value too large will create petter-panning effect.
# Increase until you observe petter-panning.
app.config.shadow_bias_scale = 1e-5

# Increase to remove global shadow acne artifact and erroneous self-shadowing.
app.config.shadow_bias_minimum = 1e-4

# app.config.shadow_pcf = 2

app.config.light_side = 1.0
app.config.light_znear = 10.0
app.config.light_zfar = 30.0

# app.config.light_up = [0.0, 1.0, 0.0]
app.simulation.sun = [0.0, 20.0, 0.0]

# Set camera pos/up
# Different methods to set dir
app.simulation.camera.pos = [0.0, 10.0, 0.0]
app.simulation.camera.up = [0.0, 0.0, 1.0]
app.simulation.camera.look_anchor()

mesh = kalast.mesh.Mesh(path="res/ico3.obj")
# print(f"vertices: {len(mesh.vertices)}")
# print(f"facets: {len(mesh.facets)}")
mesh.flatten()
mat = numpy.eye(4, dtype=numpy.float32)
mat[:3, :3] *= numpy.eye(3) * 0.2
mat[0:3, 3] = [0.0, 5.0, 0.0]
app.simulation.add_body(mesh, mat=mat)

mat = numpy.eye(4, dtype=numpy.float32)
app.simulation.add_body(mesh, mat=mat)


mat = kalast.util.mat_axis_angle(numpy.array([0.0, 0.0, 1.0]), 0.01)


def tick(sim):
    
    p0 = sim.get_matrix_model(0)[0:3, 3]
    p1 = sim.get_matrix_model(1)[0:3, 3]
    print(f"#{sim.state.iteration} {p0} {p1}")
    

    pass


app.tick = tick
app.start()
