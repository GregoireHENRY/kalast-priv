#!/usr/bin/env python

import numpy
import pandas
import spiceypy as spice

import kalast


app = kalast.app.App()

app.config.width = 1024
app.config.height = 768

app.config.global_color_mode = 3
app.config.debug_light_cube_show = False

app.config.shadow_normal_offset_scale = 2e-4
app.config.shadow_bias_scale = 1e-3
app.config.shadow_bias_minimum = 5e-4


spice.furnsh("/Users/gregoireh/data/spice/hera/kernels/mk/hera_ops_local.tm")

df = pandas.read_csv(
    "/Users/gregoireh/data/hera/tiri/tiri_images_mars_swing-by_deimos.csv"
)
images = df["image"].to_list()
et_images = df["et"].to_numpy()

et0 = spice.str2et("2025-03-12 12:40:18 UTC")
et = et0

# deltet = spice.deltet(et, "et")
# print(et, deltet)

instr = "hera_tiri"

(p_sun, _lt) = spice.spkpos("sun", et, instr, "none", instr)
(p_mars, _lt) = spice.spkpos("mars", et, instr, "none", instr)
(p_phobos, _lt) = spice.spkpos("phobos", et, instr, "none", instr)
(p_deimos, _lt) = spice.spkpos("deimos", et, instr, "none", instr)

d_sun_au = numpy.linalg.norm(p_sun) * 1e3 / kalast.util.AU
d_mars = numpy.linalg.norm(p_mars)
d_phobos = numpy.linalg.norm(p_phobos)
d_deimos = numpy.linalg.norm(p_deimos)

print(f"sun={d_sun_au:.5f}AU")
print(f"mars={d_mars:.5e}km p={p_mars}")
print(f"phobos={d_phobos:.5e}km p={p_phobos} ")
print(f"deimos={d_deimos:.5e}km p={p_deimos} ")
print()

znear = 1.0e2
zfar = 1.0e5

# add sun pos (not cube light pos) in shader for correct light dir to each facets
# create light and camera adaptative znear/zfar
# link both in a fov/proj struct? or light should use proj
# light should also use pos/up/dir like camera, and view proj calculation
app.simulation.sun = p_sun
app.config.light_distance = 1.0
app.config.light_up = [0.0, 1.0, 0.0]
app.config.light_target = p_mars
app.config.light_side = 4.0e3
app.config.light_znear = znear
app.config.light_zfar = zfar

app.simulation.camera.pos = [0.0, 0.0, 0.0]
app.simulation.camera.up = [0.0, 1.0, 0.0]
app.simulation.camera.dir = [0.0, 0.0, 1.0]
app.simulation.camera.anchor = p_mars
app.simulation.camera.set_control_none()
app.simulation.camera.projection.znear = znear
app.simulation.camera.projection.zfar = zfar
app.simulation.camera.projection.fovy = 10.0 * kalast.util.RPD
app.simulation.camera.up_world = [0.0, 1.0, 0.0]

mat_resize = numpy.eye(4)
mat_resize[:3, :3] *= kalast.entity.MARS.radii * 1e-3

mat_spin_tilt = numpy.eye(4)
mat_spin_tilt[:3, :3] = kalast.util.mat_axis_angle(
    numpy.array([0.0, 1.0, 0.0]),
    0.0,
    # kalast.util.PI
)

mat = mat_resize @ mat_spin_tilt
mat[0:3, 3] = p_mars
app.simulation.load_mesh(
    path="res/ico5.obj",
    mat=mat,
    flatten=True,
)

mat_resize = numpy.eye(4)
mat_resize[:3, :3] *= 1.0

mat = mat_resize @ mat_spin_tilt
mat[0:3, 3] = p_phobos

app.simulation.load_mesh(
    path="/Users/gregoireh/data/mesh/phobos/phobos_m003_gas_v01_simplified_10000.obj",
    mat=mat,
    flatten=True,
)

mat_resize = numpy.eye(4)
mat_resize[:3, :3] *= 1.0

mat = mat_resize @ mat_spin_tilt
mat[0:3, 3] = p_deimos

app.simulation.load_mesh(
    path="/Users/gregoireh/data/mesh/deimos/deimos_k005_tho_v02.obj",
    mat=mat,
    flatten=True,
)


def tick(sim: kalast.app.simulation.Simulation, dt: float):
    if sim.state.is_paused:
        return

    # p1 = sim.bodies[1].mat[:3, 3]
    # print(f"#{sim.state.iteration} {p1}")


app.tick = tick
app.start()
