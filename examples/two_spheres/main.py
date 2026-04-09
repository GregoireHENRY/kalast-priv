#!/usr/bin/env python

import numpy

import kalast


app = kalast.app.App()

app.config.global_color_mode = 0
app.config.debug_light_cube_show = True

app.config.shadow_normal_offset_scale = 2e-4
app.config.shadow_bias_scale = 1e-3
app.config.shadow_bias_minimum = 5e-4

app.simulation.sun = [0.0, 20.0, 0.0]
# app.config.light_up = [0.0, 1.0, 0.0]
app.config.light_side = 6.0
app.config.light_znear = 10.0
app.config.light_zfar = 30.0

app.simulation.camera.pos = [0.0, 30.0, 0.0]
app.simulation.camera.up = [0.0, 0.0, 1.0]
app.simulation.camera.look_anchor()

mat = numpy.eye(4)
app.simulation.load_mesh(path="res/ico3.obj", mat=mat, flatten=True)

mat = numpy.eye(4)
mat[:3, :3] *= numpy.eye(3) * 0.2
mat[0:3, 3] = [0.0, 5.0, 0.0]
app.simulation.load_mesh(path="res/ico3.obj", mat=mat, flatten=True)

mat = numpy.eye(4)
mat[:3, :3] = kalast.util.mat_axis_angle(numpy.array([0.0, 0.0, 1.0]), 0.01)


def tick(sim, dt):
    sim.bodies[1].mat = mat @ sim.bodies[1].mat

    # p1 = sim.bodies[1].mat[:3, 3]
    # print(f"#{sim.state.iteration} {p1}")


app.tick = tick
app.start()
