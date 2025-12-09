#!/usr/bin/env python

import time
from pathlib import Path

import numpy
from matplotlib import pyplot
import matplotlib  # noqa

import kalast
from kalast.util import AU, HOUR, DAY, RPD, STEFAN_BOLTZMANN


# Config

sun_distance = 1.0 * AU

spin_period = 6.0 * HOUR
obliquity = 0.0

albedo = 0.1
emissivity = 0.9
density = 2000.0
heat_capacity = 600.0
thermal_inertia = 400.0

dx0 = 1e-2

# Simulation starts below.

conductivity = kalast.tpm.properties.conductivity(
    thermal_inertia, density, heat_capacity
)
# diffusivity = kalast.tpm.properties.diffusivity(conductivity, density, heat_capacity)
# diffusivity = kalast._rs.tpm.properties.diffusivity()
diffusivity = kalast.tpm.properties.diffusivity()


exit()

# se = STEFAN_BOLTZMANN * prop.e

# Interior, properties, initial temperatures.
twodx0 = 2 * dx0
dx02 = dx0 * dx0
ls1 = skin_depth_1(prop.d, spin_period)
ls2pi = skin_depth_2pi(prop.d, spin_period)
ls2pi_orb = skin_depth_2pi(prop.d, orbit_period)
maxdepth = ls2pi
body.inte.z = numpy.arange(0, maxdepth + dx0, dx0)
nx = body.inte.z.size
nx_ls1 = (body.inte.z <= ls1).sum()
nx_ls2pi = (body.inte.z <= ls2pi).sum()
nx_save = (body.inte.z <= 4 * ls1).sum()
# nx_save = nx
dx = numpy.diff(body.inte.z)
dx2in = dx[:-1] * dx[:-1]
body.inte.p = numpy.ones(nx) * prop.p
body.inte.c = numpy.ones(nx) * prop.c
body.inte.k = numpy.ones(nx) * prop.k
body.inte.d = numpy.ones(nx) * prop.d
body.tmp = numpy.ones(nx) * 290.0
print(
    f"dx={dx0:.4f} ls1={ls1:.4f}({nx_ls1}) ls2pi={ls2pi:.4f}({nx_ls2pi}) ls2pi_orb={ls2pi_orb:.4f} maxdepth={maxdepth:.4f}({nx})"
)

# Time.
dt = 300
tf = 20 * DAY
nit = numpy.ceil(tf / dt).astype(int) + 1
S = stability(prop.d, dt, dx02)
print(f"Using dt={dt}, stability={S:.2f}")
print(f"simulation time={tf / DAY}days, {nit} it")

# Check convergence.
maxdt = stability_maxdt(dx02, prop.d)
print(f"max dt stable: {maxdt:.2f}")
if S > 0.5:
    raise ValueError("Stability criteria not valid.")

# Time loop progress.
progress_freq = "10"
digits = [len(_d) for _d in progress_freq.split(".")]
digits_full = 3
digits_decimal = 0
if len(digits) == 2:
    digits_decimal = digits[1]
    if digits_decimal > 0:
        digits_full += digits_decimal + 1
freqv = float(progress_freq)
last_freq_reached = -freqv
ndigits = numdigits_comma(freqv)
digit = 10**ndigits

# Saving.
t_save = 24 * HOUR
nii_save = t_save // dt
nii_hour = HOUR // dt
ts = numpy.zeros(nii_save)
tmp = numpy.zeros((nii_save, nx))
print(f"{nii_save} iterations will be recorded (frequence update: {progress_freq}%)")
print()

# to update if dt changes.
m_spin = matpow(body.spin, dt)
dtpdx2in = dt / dx2in

# Loop variables.
t = 0
it = 0

while True:
    # Get body orientation and position wrt Sun
    if it > 0:
        body.m = body.m * m_spin
    m = body.ref * body.m
    mn = glm.transpose(glm.inverse(glm.dmat3(m)))

    # For all facets, get incidence angle and distance of Sun
    p = m * body.surf.mesh.vertices[0]
    n = mn * body.surf.mesh.vertex_normals[0]
    v_sun = sun - p
    d_sun = glm.length(v_sun)
    dau_sun = d_sun / AU
    u_sun = v_sun / d_sun
    cosi = cosinc(u_sun, n)

    # Get surface flux
    sflux = sun_radiation(dau_sun, cosi, prop.a)

    # Conduction of temperature
    body.tmp[0] = newton_method(body.tmp[0], sflux, se, prop.k, body.tmp[1:3], twodx0)
    body.tmp[1:-1] = conduction_1d(body.tmp, body.inte.d, dtpdx2in)
    body.tmp[-1] = body.tmp[-2]
    if body.tmp[0] is None:
        raise ValueError("Newton method never converged.")

    # Save data
    if nit - it <= nii_save:
        ii_save = nii_save - nit + it
        ts[ii_save] = t
        tmp[ii_save] = body.tmp

    # Show progress
    progress = it / (nit - 1) * 100
    if ndigits > 0:
        progress = numpy.floor(progress * digit) / digit
    if progress >= last_freq_reached + freqv:
        last_freq_reached += freqv
        print(f"{progress:{digits_full}.{digits_decimal}f}% ({it:,}/{nit - 1:,}it)")

    # Update loop
    if t >= tf:
        break
    t += dt
    it += 1
    if it == 1:
        timer_1 = time.perf_counter()

# Final show progress
if last_freq_reached < 100:
    print(f"{100:{digits_full}.{digits_decimal}f}% ({it:,}/{nit - 1:,}it)")
print()
timer_2 = time.perf_counter()
timer_elapsed = timer_2 - timer_1
print(
    f"Simulation duration = {timer_elapsed:.4f}s ({math.floor((nit - 1) / timer_elapsed):,}it/s)"
)

# Prepare plot
ts -= ts[0]
ts /= HOUR
path_out = Path("out")
path_out.mkdir(parents=True, exist_ok=True)

kalast.plot.style.load()
fig, ax = pyplot.subplots(figsize=(6, 4))
ax.set_xlabel("Hours elapsed [h]")
ax.set_ylabel("Temperature [K]")
ax.plot(ts, tmp[:, 0], lw=1, color="k")
ax.set_xlim(0, t_save / HOUR)
# ax.set_ylim(0, None)
# ax.set_yscale("log")
# pyplot.legend()
fig.savefig(path_out / "surf.png", bbox_inches="tight", dpi=300)
# pyplot.show()

fig, ax = pyplot.subplots(figsize=(6, 4))
ax.set_xlabel("Temperature [K]")
ax.set_ylabel("Depth [cm]")
for ii in range(0, nii_save // 2, nii_hour):
    ax.plot(tmp[ii, :], body.inte.z * 100, lw=1, color="k")
# ax.set_xlim(0, None)
ax.set_ylim(0, body.inte.z[nx_save - 1] * 100)
ax.invert_yaxis()
fig.savefig(path_out / "depth_zoom.png", bbox_inches="tight", dpi=300)

ax.set_ylim(body.inte.z[-1] * 100, 0)
fig.savefig(path_out / "depth_full.png", bbox_inches="tight", dpi=300)
pyplot.show()
